use alkyd::components::Showcase;
use alkyd::workers::resources::NoiseComputeWorker;
use alkyd::AlkydPlugin;
use bevy::color::palettes::css::{BLACK, WHITE};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::AsBindGroupShaderType;
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
#[uniform(0, RampUniform)]
pub struct RampMaterial {
    stops: [ColorStop; 3],
    #[storage(1)]
    cache: [Vec4; 100],
}

#[derive(Clone, ShaderType)]
pub struct RampUniform {
    stops: [ColorStop; 3],
    cache: [Vec4; 100],
}

#[derive(Asset, TypePath, AsBindGroup, Clone, ShaderType)]
struct ColorStop {
    color: Vec3,
    position: f32,
}

#[derive(Resource, Debug)]
pub struct VoronoiImage(pub [Vec4; 100]);

impl Material for RampMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/test/test.wgsl".into()
    }
}

impl AsBindGroupShaderType<RampUniform> for RampMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<GpuImage>) -> RampUniform {
        RampUniform {
            stops: self.stops.clone(),
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
        MaterialPlugin::<RampMaterial>::default(),
    ))
    .add_systems(
        Startup,
        (init_camera.before(init_scene), create_cube, init_scene),
    )
    .add_systems(
        Update,
        (
            update_voronoi.run_if(resource_exists::<VoronoiImage>),
            rotate_mesh,
        ),
    )
    .insert_resource::<VoronoiImage>(VoronoiImage([Vec4::ZERO; 100]))
    .run();
}
#[allow(dead_code)]
fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        mesh.rotate_y(1.0 * time.delta_seconds());
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn init_scene(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        transform: Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    [
        Transform::from_xyz(1.0, 3.0, -2.0),
        Transform::from_xyz(-4.0, 0.5, -2.0),
    ]
    .map(|transform| {
        commands.spawn(PointLightBundle {
            transform,
            ..default()
        });
    });
}

pub fn create_cube(
    mut materials: ResMut<Assets<RampMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    voronoi: ResMut<VoronoiImage>,
) {
    let material = materials.add(RampMaterial {
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
        cache: voronoi.0,
    });

    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(6.0)));
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material,
            ..default()
        },
        Showcase,
    ));
}

fn update_voronoi(
    mut voronoi: ResMut<VoronoiImage>,
    compute_worker: ResMut<AppComputeWorker<NoiseComputeWorker>>,
    mut materials: ResMut<Assets<RampMaterial>>,
    cube_q: Query<&Handle<RampMaterial>>,
) {
    if !compute_worker.ready() {
        return;
    }

    if let Ok(result) = compute_worker.read_vec("centroids").as_slice().try_into() {
        let mut new_vec: [Vec4; 100] = result;
        for v_ix in 0..9 {
            for v_iy in 0..9 {
                new_vec[v_ix as usize + v_iy as usize * 10] =
                    smallest_dist(&mut result.to_vec(), v_ix, v_iy);
            }
        }
        voronoi.0 = new_vec;
    }
    if let Ok(cube) = cube_q.get_single() {
        if let Some(cube_res) = materials.get_mut(cube.id()) {
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
