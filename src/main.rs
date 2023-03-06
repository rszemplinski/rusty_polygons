use bevy::{
    prelude::*,
    render::{
        mesh::{self, Indices},
        render_resource::PrimitiveTopology,
    },
    DefaultPlugins,
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use debug_ui::DebugUIPlugin;
use marching_cubes::cpu::MarchingCubes;
use noise::{NoiseFn, SuperSimplex};

use marching_cubes::cpu::Triangle as OtherTriangle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(DebugUIPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.05,         // default: 12.0
        })
        .add_startup_system(setup)
        .run();
}

fn value_from_noise(noise: SuperSimplex, translation: Vec3) -> f32 {
    1.0 - (noise.get([
        translation.x as f64 / 32.0,
        translation.y as f64 / 32.0,
        translation.z as f64 / 32.0,
    ]) * 2.0) as f32
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let noise = SuperSimplex::new(1234);
    let mut triangles: Vec<OtherTriangle> = Vec::new();

    let iso_level = 0.5;
    let foo = MarchingCubes::new(iso_level);

    let cell_size = 1.0;
    let chunk_size = 64;

    for z in 0..chunk_size {
        for y in 0..chunk_size {
            for x in 0..chunk_size {
                let translation = Vec3::new(
                    x as f32 * cell_size - chunk_size as f32 / 2.0 * cell_size + cell_size / 2.0,
                    y as f32 * cell_size - chunk_size as f32 / 2.0 * cell_size + cell_size / 2.0,
                    z as f32 * cell_size - chunk_size as f32 / 2.0 * cell_size + cell_size / 2.0,
                );

                let cell_points = [
                    translation + Vec3::new(-cell_size / 2.0, -cell_size / 2.0, -cell_size / 2.0),
                    translation + Vec3::new(cell_size / 2.0, -cell_size / 2.0, -cell_size / 2.0),
                    translation + Vec3::new(cell_size / 2.0, -cell_size / 2.0, cell_size / 2.0),
                    translation + Vec3::new(-cell_size / 2.0, -cell_size / 2.0, cell_size / 2.0),
                    translation + Vec3::new(-cell_size / 2.0, cell_size / 2.0, -cell_size / 2.0),
                    translation + Vec3::new(cell_size / 2.0, cell_size / 2.0, -cell_size / 2.0),
                    translation + Vec3::new(cell_size / 2.0, cell_size / 2.0, cell_size / 2.0),
                    translation + Vec3::new(-cell_size / 2.0, cell_size / 2.0, cell_size / 2.0),
                ];

                let cell_points = [
                    (cell_points[0], value_from_noise(noise, cell_points[0])),
                    (cell_points[1], value_from_noise(noise, cell_points[1])),
                    (cell_points[2], value_from_noise(noise, cell_points[2])),
                    (cell_points[3], value_from_noise(noise, cell_points[3])),
                    (cell_points[4], value_from_noise(noise, cell_points[4])),
                    (cell_points[5], value_from_noise(noise, cell_points[5])),
                    (cell_points[6], value_from_noise(noise, cell_points[6])),
                    (cell_points[7], value_from_noise(noise, cell_points[7])),
                ];
                triangles.append(&mut foo.polygonize(cell_points));
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let vertices = triangles
        .iter()
        .map(|triangle| [triangle.vertex_1, triangle.vertex_2, triangle.vertex_3])
        .flatten()
        .map(|vector| [vector.x, vector.y, vector.z])
        .collect::<Vec<_>>();

    let indices = (0..vertices.len())
        .map(|index| index as u32)
        .collect::<Vec<u32>>();

    let mut normals: Vec<[f32; 3]> = Vec::new();

    for triangle in indices.chunks(3) {
        let a = Vec3::from(vertices[(triangle)[0] as usize]);
        let b = Vec3::from(vertices[(triangle)[1] as usize]);
        let c = Vec3::from(vertices[(triangle)[2] as usize]);

        let normal = (b - a).cross(c - a).normalize();

        normals.push(normal.into());
        normals.push(normal.into());
        normals.push(normal.into());
    }

    mesh.set_indices(Some(Indices::U32(indices)));

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::AQUAMARINE,
            perceptual_roughness: 0.8,
            ..default()
        }),
        ..default()
    });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 10. })),
    //     material: materials.add(Color::BLUE.into()),
    //     ..default()
    // });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
            shadow_projection: OrthographicProjection {
                left: -10.0,
                right: 10.0,
                bottom: -10.0,
                top: 10.0,
                near: -50.0,
                far: 50.0,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCam);
}
