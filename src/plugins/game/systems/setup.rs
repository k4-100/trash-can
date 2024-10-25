use bevy::prelude::*;

use crate::{spawn_cube_with_standard_material, utils};

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
                translation: Vec3::new(0.0, 0.0, 0.0),
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
    spawn_cube_with_standard_material!(
        commands,
        meshes,
        materials.add(Color::srgb_u8(0, 0, 255)),
        Vec3::new(200.0, 200.0, 200.0),
        Transform::from_xyz(0.0, 0.0, -1000.0)
    );

    spawn_cube_with_standard_material!(
        commands,
        meshes,
        materials.add(Color::srgb_u8(0, 255, 0)),
        Vec3::new(200.0, 200.0, 200.0),
        Transform::from_xyz(1000.0, 0.0, 0.0)
    );

    spawn_cube_with_standard_material!(
        commands,
        meshes,
        materials.add(Color::srgb_u8(255, 0, 0)),
        Vec3::new(200.0, 200.0, 200.0),
        Transform::from_xyz(0.0, 0.0, 1000.0)
    );

    spawn_cube_with_standard_material!(
        commands,
        meshes,
        materials.add(Color::srgb_u8(127, 0, 127)),
        Vec3::new(200.0, 200.0, 200.0),
        Transform::from_xyz(-1000.0, 0.0, 0.0)
    );

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::from_size(Vec3::new(200.0, 200.0, 200.0))),
    //     material: materials.add(Color::srgb_u8(0, 0, 255)),
    //     transform: Transform::from_xyz(0.0, 0.0, -1000.0),
    //     ..default()
    // });
    //
    //
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::from_size(Vec3::new(200.0, 200.0, 200.0))),
    //     material: materials.add(Color::srgb_u8(0, 0, 255)),
    //     transform: Transform::from_xyz(0.0, 0.0, -1000.0),
    //     ..default()
    // });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::from_size(Vec3::new(200.0, 200.0, 200.0))),
    //     material: materials.add(Color::srgb_u8(0, 0, 255)),
    //     transform: Transform::from_xyz(0.0, 0.0, -1000.0),
    //     ..default()
    // });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::from_size(Vec3::new(200.0, 200.0, 200.0))),
    //     material: materials.add(Color::srgb_u8(0, 0, 255)),
    //     transform: Transform::from_xyz(0.0, 0.0, -1000.0),
    //     ..default()
    // });

    // let square_mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));
    //
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: square_mesh_handle,
    //     material: materials.add(Color::srgba(0.0, 255.0, 255.0, 1.0)),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
}
