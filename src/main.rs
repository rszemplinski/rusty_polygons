use bevy::{
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
    DefaultPlugins,
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};

use marching_cubes::cpu::march;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(Mesh)

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0., 0., 0.], [1., 2., 1.], [2., 0., 0.]],
    );

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCam);
}
