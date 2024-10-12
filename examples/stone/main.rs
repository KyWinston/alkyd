use alkyd::components::Showcase;
use alkyd::workers::resources::NoiseComputeWorker;
use alkyd::AlkydPlugin;
use bevy::asset::LoadState;
use bevy::color::palettes::css::{BLACK, WHITE};
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
// use bevy::core_pipeline::Skybox;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
    AsBindGroupShaderType, TextureViewDescriptor, TextureViewDimension,
};
use bevy::render::texture::GpuImage;
use bevy::{
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
        texture::{ImageAddressMode, ImageSamplerDescriptor},
    },
};
use bevy_easy_compute::prelude::AppComputeWorker;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, TerrazoUniform)]
pub struct TerrazoMaterial {
    stops: [ColorStop; 3],
    scale: f32,
    lc_scale: f32,
    sc_scale: f32,
    lc_size: f32,
    sc_size: f32,
    color_1: Color,
    color_2: Color,
    roughness: f32,
    #[storage(1)]
    cache: [Vec4; 1000],
}

#[derive(Clone, ShaderType)]
pub struct TerrazoUniform {
    stops: [ColorStop; 3],
    scale: f32,
    lc_scale: f32,
    sc_scale: f32,
    lc_size: f32,
    sc_size: f32,
    color_1: Vec4,
    color_2: Vec4,
    roughness: f32,
    cache: [Vec4; 1000],
}

#[derive(Asset, TypePath, AsBindGroup, Clone, ShaderType)]
struct ColorStop {
    color: Vec3,
    position: f32,
}

#[derive(Resource)]
struct Cubemap(Handle<Image>, Handle<Image>);

#[derive(Resource, Debug)]
pub struct VoronoiImage(pub [Vec4; 1000]);

impl Material for TerrazoMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/surfaces/terrazo.wgsl".into()
    }
}

impl AsBindGroupShaderType<TerrazoUniform> for TerrazoMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> TerrazoUniform {
        TerrazoUniform {
            stops: self.stops.clone(),
            scale: self.scale,
            lc_scale: self.lc_scale,
            sc_scale: self.sc_scale,
            lc_size: self.lc_size,
            sc_size: self.sc_size,
            color_1: self.color_1.to_linear().to_vec4(),
            color_2: self.color_2.to_linear().to_vec4(),
            roughness: self.roughness,
            cache: self.cache,
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                address_mode_w: ImageAddressMode::Repeat,
                ..Default::default()
            },
        }),
        AlkydPlugin { debug: true },
        MaterialPlugin::<ConcreteMaterial>::default(),
        MaterialPlugin::<TerrazoMaterial>::default(),
    ))
    .add_systems(PreStartup, init_cubemap)
    .add_systems(
        Startup,
        (init_camera.before(init_scene), create_cube, init_scene),
    )
    .add_systems(
        Update,
        (
            create_skybox.run_if(resource_exists::<Cubemap>),
            update_voronoi.run_if(resource_exists::<VoronoiImage>),
            rotate_mesh,
        ),
    )
    .insert_resource(Msaa::Off)
    .insert_resource::<VoronoiImage>(VoronoiImage([Vec4::ZERO; 1000]))
    .run();
}

fn init_cubemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("StandardCubeMap.png");
    let image_diffuse = asset_server.load("StandardCubeMap_diffuse.png");

    commands.insert_resource(Cubemap(image.clone(), image_diffuse.clone()));
}

fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    for mut mesh in mesh_q.iter_mut() {
        mesh.rotate_y(1.0 * time.delta_seconds());
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ScreenSpaceAmbientOcclusionBundle::default(),
    ));
}

fn init_scene(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    [
        Transform::from_xyz(1.0, 3.0, -2.0),
        Transform::from_xyz(-4.0, 0.5, -2.0),
    ]
    .map(|transform| {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 100_000.0,
                shadows_enabled: true,
                ..default()
            },
            transform,
            ..default()
        });
    });
}

fn create_skybox(
    mut commands: Commands,
    cubemap: Res<Cubemap>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    cam_q: Query<Entity, With<Camera3d>>,
) {
    if asset_server.load_state(&cubemap.0) == LoadState::Loaded {
        let image = images.get_mut(&cubemap.0).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }
        let image_2 = images.get_mut(&cubemap.1).unwrap();
        if image_2.texture_descriptor.array_layer_count() == 1 {
            image_2.reinterpret_stacked_2d_as_array(image_2.height() / image_2.width());
            image_2.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }
        if let Ok(cam) = cam_q.get_single() {
            commands.entity(cam).insert((
                // Skybox {
                //     image: cubemap.0.clone(),
                //     brightness: 1000.0,
                // },
                EnvironmentMapLight {
                    diffuse_map: cubemap.1.clone(),
                    specular_map: cubemap.0.clone(),
                    intensity: 20.0,
                },
            ));
        }
    }
}

pub fn create_cube(
    mut terrazo: ResMut<Assets<TerrazoMaterial>>,
    mut concrete: ResMut<Assets<ConcreteMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    voronoi: ResMut<VoronoiImage>,
) {
    let material = terrazo.add(TerrazoMaterial {
        stops: [
            ColorStop {
                color: BLACK.to_vec3(),
                position: 0.35,
            },
            ColorStop {
                color: WHITE.to_vec3(),
                position: 0.48,
            },
            ColorStop {
                color: WHITE.to_vec3(),
                position: 1.0,
            },
        ],
        scale: 20.0,
        lc_scale: 0.5,
        sc_scale: 1.0,
        lc_size: 1.0,
        sc_size: 1.0,
        color_1: Color::linear_rgba(0.43, 0.45, 0.44, 1.0),
        color_2: Color::linear_rgba(0.25, 0.25, 0.25, 1.0),
        roughness: 0.1,
        cache: voronoi.0,
    });

    let material_2 = concrete.add(ConcreteMaterial {
        stops: [
            ColorStop {
                color: Color::linear_rgb(0.2, 0.2, 0.2).to_linear().to_vec3(),
                position: 0.0,
            },
            ColorStop {
                color: Color::linear_rgb(0.35, 0.35, 0.35).to_linear().to_vec3(),
                position: 0.4,
            },
            ColorStop {
                color: Color::linear_rgb(0.5, 0.5, 0.5).to_linear().to_vec3(),
                position: 1.0,
            },
        ],
        scale: 400.0,
        cache: voronoi.0,
    });

    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(6.0)));
    commands.spawn((
        MaterialMeshBundle {
            mesh: mesh.clone(),
            material,
            ..default()
        },
        Showcase,
    ));

    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material: material_2,
            transform: Transform::from_translation(Vec3::new(12.0, 0.0, 0.0)),
            ..default()
        },
        Showcase,
    ));
}

fn update_voronoi(
    mut voronoi: ResMut<VoronoiImage>,
    compute_worker: ResMut<AppComputeWorker<NoiseComputeWorker>>,
    mut terrazo: ResMut<Assets<TerrazoMaterial>>,
    t_cube_q: Query<&Handle<TerrazoMaterial>>,
    mut concrete: ResMut<Assets<ConcreteMaterial>>,
    c_cube_q: Query<&Handle<ConcreteMaterial>>,
) {
    if !compute_worker.ready() {
        return;
    }

    if let Ok(result) = compute_worker.read_vec("centroids").as_slice().try_into() {
        let mut new_vec: [Vec4; 1000] = result;
        for v_ix in 0..9 {
            for v_iy in 0..9 {
                new_vec[v_ix as usize + v_iy as usize * 100] =
                    smallest_dist(&mut result.to_vec(), v_ix, v_iy);
            }
        }
        voronoi.0 = new_vec;
    }
    if let Ok(cube) = t_cube_q.get_single() {
        if let Some(cube_res) = terrazo.get_mut(cube.id()) {
            cube_res.cache = voronoi.0;
        }
    }
    if let Ok(cube) = c_cube_q.get_single() {
        if let Some(cube_res) = concrete.get_mut(cube.id()) {
            cube_res.cache = voronoi.0;
        }
    }
}

fn smallest_dist(points: &mut Vec<Vec4>, idx: i32, idy: i32) -> Vec4 {
    let mut min_dist = 1.0;
    for x in -1..1 {
        for y in -1..1 {
            let neighbor_dist = points[(idx + x).abs() as usize + (idy + y).abs() as usize * 10]
                .xy()
                .distance(points[idx as usize + idy as usize * 10].xy());
            if neighbor_dist < min_dist {
                min_dist = neighbor_dist;
            }
        }
    }
    points[idx as usize + idy as usize * 10].w = min_dist;
    points[idx as usize + idy as usize * 10]
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[uniform(0, ConcreteUniform)]
pub struct ConcreteMaterial {
    stops: [ColorStop; 3],
    scale: f32,
    #[storage(1)]
    cache: [Vec4; 1000],
}

#[derive(Clone, ShaderType)]
pub struct ConcreteUniform {
    stops: [ColorStop; 3],
    scale: f32,
    cache: [Vec4; 1000],
}

impl Material for ConcreteMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/surfaces/concrete.wgsl".into()
    }
}

impl AsBindGroupShaderType<ConcreteUniform> for ConcreteMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> ConcreteUniform {
        ConcreteUniform {
            stops: self.stops.clone(),
            scale: self.scale,
            cache: self.cache,
        }
    }
}
