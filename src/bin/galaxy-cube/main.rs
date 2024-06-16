use alkyd::{
    materials::{
        galaxyfog::galaxy::GalaxyFogMaterial,
        painterly::resources::{MaterialsInspector, PainterlyInspector, VoronoiImage},
    },
    utilities::systems::LoadNoise,
    AlkydPlugin, Showcase,
};

use bevy::{
    color::palettes::css::PURPLE,
    core_pipeline::prepass::NormalPrepass,
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..Default::default()
                },
            }),
            AlkydPlugin { debug: false },
        ))
        .add_systems(Startup, (init_camera.before(init_scene), init_scene))
        .add_systems(
            Update,
            (
                rotate_mesh.run_if(resource_exists::<PainterlyInspector>),
                create_cube.run_if(resource_added::<VoronoiImage>),
            ),
        )
        .run();
}

fn rotate_mesh(
    mut mesh_q: Query<&mut Transform, With<Showcase>>,
    inspector: Res<MaterialsInspector>,
    time: Res<Time>,
) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        if inspector.turn_table {
            mesh.rotate_y(1.0 * time.delta_seconds());
        }
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        NormalPrepass,
    ));
}

fn init_scene(mut commands: Commands, mut ev: EventWriter<LoadNoise>) {
    ev.send(LoadNoise);
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
    mut materials: ResMut<Assets<GalaxyFogMaterial>>,
    _voro: Res<VoronoiImage>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    _asset_server: Res<AssetServer>,
) {
    let material = materials.add(GalaxyFogMaterial {
        diffuse_color: Color::srgb_from_array(PURPLE.to_f32_array_no_alpha()),
        radius: 2.0,
        ..default()
    });

    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(10.0)));
    commands.spawn((MaterialMeshBundle {
        mesh,
        material,
        ..default()
    },));
}
