use alkyd::components::Showcase;
use bevy::{color::palettes::css::LAWN_GREEN, prelude::*, scene::SceneInstance};
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};
use iyes_perf_ui::prelude::PerfUiDefaultEntries;
use rand::{
    distr::{self, Uniform},
    Rng,
};

pub fn rotate_mesh(mut mesh_q: Query<&mut Transform, With<Showcase>>, time: Res<Time>) {
    if let Ok(mut mesh) = mesh_q.get_single_mut() {
        mesh.rotate_y(1.0 * time.delta_secs());
    }
}

pub fn init_scene(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(1.0, 3.0, -2.0)));
    commands.spawn((PointLight::default(), Transform::from_xyz(-4.0, 0.5, -2.0)));
    commands.spawn((
        Camera3d::default(),
        ThirdPersonCamera {
            zoom: Zoom::new(15.0, 60.0),
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn create_plane(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    server: Res<AssetServer>,
) {
    let material = materials.add(StandardMaterial {
        base_color: LAWN_GREEN.into(),
        cull_mode: None,
        ..default()
    });

    let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(7.5)));
    commands.spawn((
        Mesh3d(mesh),
        ThirdPersonCameraTarget,
        MeshMaterial3d(material.clone()),
    ));
    let mut rng = rand::rng();
    let x = distr::Uniform::new_inclusive(-7.5, 7.5).unwrap();

    let glb = server.load(GltfAssetLabel::Scene(0).from_asset("example_assets/grass.glb"));
    for _ in 0..10000 {
        commands.spawn((
            SceneRoot(glb.clone()),
            Transform::from_xyz(rng.sample(x), -0.1, rng.sample(x)),
        ));
    }
}

pub fn add_grass_material(
    mut commands: Commands,
    mut root: Query<Entity, Added<SceneInstance>>,
    grass: Query<(Entity, &Parent)>,
    blade: Query<(Entity, &Parent, &Name)>,
    mut b_mesh: Query<(Entity, &Parent, &mut Transform), With<Mesh3d>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: LAWN_GREEN.into(),
        cull_mode: None,
        ..default()
    });
    let mut rng = rand::rng();

    let rot: Uniform<f32> = distr::Uniform::new_inclusive(-180.0, 180.0).unwrap();

    for ent in root.iter_mut() {
        for grass in grass.iter() {
            if **grass.1 == ent {
                for blade in blade.iter() {
                    if **blade.1 == grass.0 {
                        for mut b in b_mesh.iter_mut() {
                            if **b.1 == blade.0 {
                                commands
                                    .entity(b.0)
                                    .insert(MeshMaterial3d(material.clone()));
                                b.2.rotate_local_y(rng.sample(rot).to_radians());
                            }
                        }
                    }
                }
            }
        }
        // commands
        //     .entity(blade)
        //     .insert(MeshMaterial3d(materials.add(material.clone())));
    }
}
