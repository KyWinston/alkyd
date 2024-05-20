use alkyd::{materials::painterly::PainterlyMaterial, AlkydPlugin};
use bevy::{
    app::{App, Startup},
    asset::Assets,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::system::{Commands, Res, ResMut, Resource},
    math::{primitives::Cuboid, Vec3},
    pbr::MaterialMeshBundle,
    prelude::{default, *},
    render::mesh::Mesh,
    transform::components::Transform,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Resource)]
pub struct BenchSettings(i32, bool);

fn run(n: i32, m: bool) {
    App::new()
        .insert_resource::<BenchSettings>(BenchSettings(n, m))
        .add_plugins((
            MinimalPlugins,
            EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            },
            AlkydPlugin { debug: false },
        ))
        .add_systems(Startup, setup);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    num: Res<BenchSettings>,
    mut materials: ResMut<Assets<PainterlyMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    for x in 0..num.0 {
        for y in 0..num.0 {
            for z in 0..num.0 {
                if num.1 {
                    commands.spawn(MaterialMeshBundle {
                        mesh: meshes.add(Cuboid::from_size(Vec3::splat(1.0))),
                        material: materials.add(PainterlyMaterial {
                            distort: 4.0,
                            brush_handle: None,
                            brush_handle_normal: None,
                            voro_cache: None,
                            ..default()
                        }),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    });
                } else {
                    commands.spawn(MaterialMeshBundle {
                        mesh: meshes.add(Cuboid::from_size(Vec3::splat(1.0))),
                        material: standard_materials.add(StandardMaterial::default()),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    });
                }
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cube scatter", |f| {
        f.iter(|| run(black_box(100), black_box(false)))
    });
    c.bench_function("cube scatter trippy", |f| {
        f.iter(|| run(black_box(100), black_box(true)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
