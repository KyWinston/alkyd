use alkyd::{
    components::Showcase,
    foliage::node::{FoliageMaterial, FoliageProperties},
};

use bevy::{
    color::palettes::css::{DARK_GREEN, GREEN},
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::{
        mesh::{MeshTag, PlaneMeshBuilder},
        render_resource::AsBindGroup,
        storage::ShaderStorageBuffer,
    },
    scene::SceneInstanceReady,
};

use bevy_app_compute::prelude::ShaderRef;
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};
use iyes_perf_ui::prelude::PerfUiDefaultEntries;
use rand::Rng;

pub fn init_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PerfUiDefaultEntries::default());
    commands.spawn((
        DirectionalLight {
            illuminance: 25000.0,
            ..default()
        },
        Transform::from_xyz(-4.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn_batch([
        (PointLight::default(), Transform::from_xyz(1.0, 3.0, -2.0)),
        (PointLight::default(), Transform::from_xyz(-4.0, 0.5, -2.0)),
    ]);
    commands.spawn((
        Camera3d::default(),
        ThirdPersonCamera {
            zoom: Zoom::new(5.0, 40.0),
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let asset =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("example_assets/grass_blades.gltf"));
    commands
        .spawn((
            Name::new("blade_instance"),
            Visibility::Hidden,
            SceneRoot(asset),
        ))
        .observe(create_grass);
}

pub fn create_grass(
    _trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ExtendedMaterial<FoliageMaterial, GrassMaterial>>>,
    mut s_materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    blade: Query<&Mesh3d, With<ChildOf>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let plane = meshes.add(PlaneMeshBuilder::new(Dir3::Y, Vec2::splat(5.0)).subdivisions(4));
    let tris = meshes
        .get(plane.id())
        .unwrap()
        .triangles()
        .expect("invalid mesh");
    let mut points: Vec<Vec3> = vec![];
    let properties = FoliageProperties {
        density: 300.0,
        count: 30,
        variance: rand::thread_rng().gen_range(0.0..3.14),
        color: DARK_GREEN.to_vec4(),
    };
    let mut tri_count = 0;
    for tri in tris.into_iter() {
        let A = tri.vertices[0];
        let B = tri.vertices[1];
        let C = tri.vertices[2];
        tri_count += 1;
        let AB = Vec3::new(B.x - A.x, B.y - A.y, B.z - A.z);
        let AC = Vec3::new(C.x - A.x, C.y - A.y, C.z - A.z);
        for _p in 0..properties.count {
            let mut R = rand::thread_rng().gen_range(0.0..1.0);
            let mut S = rand::thread_rng().gen_range(0.0..1.0);
            if R + S >= 1.0 {
                R = 1.0 - R;
                S = 1.0 - S;
            }
            let rand_pos = (A + R * AB + S * AC) + Vec3::new(0.0, 0.25, 0.0);
            points.push(rand_pos);
        }
    }

    let grass_mat = ExtendedMaterial {
        base: FoliageMaterial {
            properties: properties.clone(),
            points: buffers.add(ShaderStorageBuffer::from(points)),
        },
        extension: GrassMaterial { displacement: None },
    };

    commands.spawn((
        Mesh3d(plane),
        Showcase,
        MeshMaterial3d(s_materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..default()
        })),
        Transform::default(),
        ThirdPersonCameraTarget,
    ));
    if let Ok(blade_inst) = blade.single() {
        for i in 0..properties.count * tri_count {
            commands.spawn((
                Mesh3d(blade_inst.0.clone()),
                MeshTag(i),
                MeshMaterial3d(materials.add(grass_mat.clone())),
            ));
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Default, Clone)]
pub struct GrassMaterial {
    #[texture(100)]
    #[sampler(101)]
    pub displacement: Option<Handle<Image>>,
}

impl MaterialExtension for GrassMaterial {
    fn fragment_shader() -> ShaderRef {
        "example_assets/grass.wgsl".into()
    }
}
