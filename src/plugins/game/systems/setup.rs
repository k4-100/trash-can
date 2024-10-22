use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        transform: Transform {
            scale: Vec3 {
                x: 1.5,
                y: 1.5,
                z: 1.0,
            },
            translation: Vec3::new(0.0, 0.0, -5.0),
            ..default()
        },
        ..default()
    },));
}

pub fn setup_square(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let square_mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: square_mesh_handle,
        material: materials.add(Color::srgba(0.0, 255.0, 255.0, 1.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
