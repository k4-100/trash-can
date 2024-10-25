use bevy::prelude::*;

use crate::utils;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        utils::components::CurrentPlayer,
        utils::components::Player,
        Camera3dBundle {
            transform: Transform {
                scale: Vec3 {
                    x: 1.5,
                    y: 1.5,
                    z: 1.0,
                },
                translation: Vec3::new(0.0, 0.0, 500.0),
                ..default()
            }
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
    ));
}

pub fn setup_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::new(200.0, 200.0, 200.0))),
        // material: materials.add(StandardMaterial {
        //     base_color: Color::srgba(0.0, 255.0, 255.0, 1.0),
        //     ..default()
        // }),
        // material: materials.add(Color::srgb(0.0, 0.0, 255.0)),
        material: materials.add(Color::srgb_u8(0, 0, 255)),
        transform: Transform::from_xyz(0.0, 0.0, -200.0),
        ..default()
    });
    // let square_mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));
    //
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: square_mesh_handle,
    //     material: materials.add(Color::srgba(0.0, 255.0, 255.0, 1.0)),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
}
