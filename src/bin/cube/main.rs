use alkyd::{materials::painterly::{components::Showcase, painterly::PainterlyMaterial, resources::{MaterialsInspector, PainterlyInspector, VoronoiImage}}, utilities::systems::LoadNoise, AlkydPlugin};

use bevy::{
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
            AlkydPlugin { debug: true },
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
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 3.0, -2.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-4.0, 0.5, -2.0),
        ..default()
    });
}

pub fn create_cube(
    mut materials: ResMut<Assets<PainterlyMaterial>>,
    voro: Res<VoronoiImage>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let material = materials.add(PainterlyMaterial {
        diffuse_color: Color::BLUE,
        brush_handle: Some(asset_server.load("brush_grunge.png")),
        brush_handle_normal: Some(asset_server.load("brush_grunge_normal.png")),
        voro_cache: voro.0.clone(),
        ..default()
    });

    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(4.0)));
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material,
            ..default()
        },
        Showcase,
    ));
}
