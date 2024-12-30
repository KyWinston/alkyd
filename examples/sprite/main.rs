use alkyd::components::Showcase;
use alkyd::materials::spritely::components::{Animation, AnimationData};
use alkyd::materials::spritely::shader::SpritelyMaterial;

use alkyd::AlkydPlugin;

use bevy::image::{ImageAddressMode, ImageSamplerDescriptor};
use bevy::pbr::ScreenSpaceAmbientOcclusion;

use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

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
    ))
    .add_systems(
        Startup,
        (init_camera.before(init_scene), create_cube, init_scene),
    )
    .add_systems(Update, (rotate_mesh,))
    .run();
}

fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    for mut mesh in mesh_q.iter_mut() {
        mesh.rotate_y(1.0 * time.delta_secs());
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Msaa::Off,
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ScreenSpaceAmbientOcclusion::default(),
    ));
}

fn init_scene(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    [
        Transform::from_xyz(1.0, 3.0, -2.0),
        Transform::from_xyz(-4.0, 0.5, -2.0),
    ]
    .map(|transform| {
        commands.spawn((
            PointLight {
                intensity: 100_000.0,
                shadows_enabled: true,
                ..default()
            },
            transform,
        ));
    });
}

pub fn create_cube(
    mut sprite: ResMut<Assets<SpritelyMaterial>>,
    server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1.0)));
    let mut animations: HashMap<String, Animation> = HashMap::new();
    animations.extend(
        [("idle", 0, 0, 18), ("run", 4, 10, 9), ("jog", 4, 0, 9)].map(|f| {
            (
                f.0.to_string(),
                Animation {
                    start_indices: [f.1, f.2],
                    anim_length: f.3,
                },
            )
        }),
    );
    commands.spawn((
        Mesh3d(mesh.clone()),
        AnimationData::new([8, 18], 8, animations, 12),
        MeshMaterial3d(sprite.add(SpritelyMaterial {
            sheet_mask: Some(server.load("warrior/full_sheet.png")),
            color_uv: Some(server.load("warrior/uv_sheet.png")),
            normal_map: Some(server.load("warrior/normal_sheet.png")),
            ao_map: Some(server.load("warrior/occlusion.png")),
            ..default()
        })),
        // Showcase,
    ));
}
