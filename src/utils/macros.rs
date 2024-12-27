use bevy::prelude::*;

#[macro_export]
macro_rules! spawn_cube_with_standard_material {
    (
        $commands:expr,
        $meshes:expr,
        $material:expr,
        $size_vec3:expr,
        $transform:expr
    ) => {{
        // $commands.spawn(PbrBundle {
        //     mesh: $meshes.add(Cuboid::from_size($size_vec3)),
        //     material: $material,
        //     transform: $transform,
        //     ..default()
        // });
        $commands.spawn((
            Mesh3d($meshes.add(Cuboid::from_size($size_vec3))),
            MeshMaterial3d($material),
            Transform::from($transform),
        ));
    }};
}

// #[macro_export]
// macro_rules! spawn_cubes_with_standard_material {
//     (
//         $commands:expr,
//         $meshes:expr,
//         $material:expr,
//         $size_vec3:expr,
//         $transform:expr
//     ) => {{
//         $commands.spawn(PbrBundle {
//             mesh: $meshes.add(Cuboid::from_size($size_vec3)),
//             material: $material,
//             transform: $transform,
//             ..default()
//         });
//     }};
// }

// #[macro_export]
// macro_rules! spawn_sprite_bundle {
//     (
//         $commands:expr,
//         $color:expr,
//         $transform:expr,
//     ) => {{
//         $commands.spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(0.25, 0.25, 0.75),
//                 custom_size: Some(Vec2::new(50.0, 100.0)),
//                 ..default()
//             },
//             transform: Transform::from_translation(Vec3::new(-0., 0., 0.)),
//             ..default()
//         });
//     }};
// }
