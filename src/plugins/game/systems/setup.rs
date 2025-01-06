use std::fs;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_render::camera::Viewport;

use crate::{spawn_cube_with_standard_material, utils::*};

use map_generator;

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_camera = commands
        .spawn((
            components::CurrentPlayer,
            components::Player,
            Camera3d::default(),
            Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2 { x: 1, y: 1 },
                    ..default()
                }),
                order: 1,
                // clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.0, 0.0, 255.0)),
                ..default()
            },
            Transform {
                scale: Vec3 {
                    x: 2.0,
                    y: 2.0,
                    z: 1.0,
                },
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            }
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            Friction::coefficient(0.7),
            Restitution::coefficient(0.3),
            RigidBody::Dynamic,
            // LockedAxes::ROTATION_LOCKED
            LockedAxes::ROTATION_LOCKED,
            Collider::cuboid(20.0, 20.0, 20.0),
            Velocity {
                linvel: Vec3::new(0.0, 0.0, 0.0),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            },
        ))
        .id();

    let minimap_camera = commands
        .spawn((
            components::MiniMapCamera,
            Camera3d::default(),
            Transform {
                scale: Vec3 {
                    x: 4.0,
                    y: 4.0,
                    z: 1.0,
                },
                translation: Vec3::new(0.0, 2000.0, 0.0),
                ..default()
            }
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(364, 364),
                    ..default()
                }),
                order: 2,
                // clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.0, 0.0, 255.0)),
                ..default()
            },
        ))
        .id();

    let gun = commands
        .spawn((
            Transform {
                translation: Vec3::new(-15.0, -10.0, -60.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                rotation: Quat::from_scaled_axis(Vec3::new(0.0, -1.5, 0.0)),
            },
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/untitled.glb")),
            ),
        ))
        .id();

    commands
        .spawn((
            TargetCamera(main_camera),
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            // BackgroundColor(Color::srgb(0.15, 0.45, 0.15)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Node {
                    width: Val::Px(5.0),
                    height: Val::Px(5.0),

                    // row_gap: Val::Px(5.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.95, 0.15)),
            ));
        });

    commands.spawn((
        TargetCamera(minimap_camera),
        components::CoordsText,
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new(" x: -100.0\ny: 50.5"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/HackNerdFont-Regular.ttf"),
            font_size: 30.0,

            ..default()
        },
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::Z),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(400.0),
            right: Val::Px(200.0),
            ..default()
        },
    ));

    commands.entity(main_camera).add_children(&[gun]);
}

// pub fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {}

pub fn setup_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn_cube_with_standard_material!(
    //     commands,
    //     meshes,
    //     materials.add(Color::srgb_u8(0, 0, 255)),
    //     Vec3::new(200.0, 200.0, 200.0),
    //     Transform::from_xyz(0.0, 0.0, -1000.0)
    // );
    //
    // spawn_cube_with_standard_material!(
    //     commands,
    //     meshes,
    //     materials.add(Color::srgb_u8(0, 255, 0)),
    //     Vec3::new(200.0, 200.0, 200.0),
    //     Transform::from_xyz(1000.0, 0.0, 0.0)
    // );
    //
    // spawn_cube_with_standard_material!(
    //     commands,
    //     meshes,
    //     materials.add(Color::srgb_u8(255, 0, 0)),
    //     Vec3::new(200.0, 200.0, 200.0),
    //     Transform::from_xyz(0.0, 0.0, 1000.0)
    // );
    //
    // spawn_cube_with_standard_material!(
    //     commands,
    //     meshes,
    //     materials.add(Color::srgb_u8(127, 0, 127)),
    //     Vec3::new(200.0, 200.0, 200.0),
    //     Transform::from_xyz(-1000.0, 0.0, 0.0)
    // );

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

pub fn setup_block_from_txt(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut camera_transform: Query<&mut Transform, With<components::CurrentPlayer>>,
) {
    // let map_raw = fs::read_to_string("assets/map.txt").expect("WRONG PATH");
    let map_raw = map_generator::generate_map();

    let map: Vec<&str> = map_raw.trim_end().split("\n").collect();

    // println!("{:#?}", map);

    // commands.spawn((
    //     DirectionalLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     // This is a relatively small scene, so use tighter shadow
    //     // cascade bounds than the default for better quality.
    //     // We also adjusted the shadow map to be larger since we're
    //     // only using a single cascade.
    //     CascadeShadowConfigBuilder {
    //         num_cascades: 1,
    //         maximum_distance: 1.6,
    //         ..default()
    //     }
    //     .build(),
    // ));

    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, 100.0, -800.0),
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/untitled.glb"))),
    ));

    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("models/bgun.gltf"),
    //     transform: Transform::from_xyz(200.0, 400.0, 200.0),
    //     ..default()
    // });

    // spawn_cube_with_standard_material!(
    //     commands,
    //     meshes,
    //     materials.add(Color::srgb_u8(0, 255, 127)),
    //     Vec3::new(200.0, 200.0, 200.0),
    //     Transform::from_xyz(0.0, 100.0, -800.0)
    // );

    spawn_cube_with_standard_material!(
        commands,
        meshes,
        materials.add(Color::srgb_u8(255, 0, 127)),
        Vec3::new(200.0 * map[0].len() as f32, 200.0, 200.0 * map.len() as f32),
        Transform::from_xyz(
            200.0 * (map[0].len() / 2) as f32 + 100.0,
            -200.0,
            200.0 * (map.len() / 2) as f32 - 100.0,
        )
    );

    for (y, map_row) in map.iter().enumerate() {
        for (x, map_sign) in map_row.split("").enumerate() {
            if map_sign == "#" {
                spawn_cube_with_standard_material!(
                    commands,
                    meshes,
                    materials.add(Color::srgb_u8(127, 0, 127)),
                    Vec3::new(200.0, 200.0, 200.0),
                    Transform::from_xyz(200.0 * x as f32, 0.0, 200.0 * y as f32)
                );
            }

            if map_sign == "P" {
                let mut cmr_transform = camera_transform.single_mut();
                // cmr_transform.translation = *Vec3::new(
                //     200.0 * x as f32,
                //     cmr_transform.translation.y,
                //     200.0 * y as f32,
                // );
                cmr_transform.translation.x = 200.0 * x as f32;
                cmr_transform.translation.z = 200.0 * y as f32;
            }

            if map_sign == "E" {
                commands
                    .spawn((
                        Transform {
                            translation: Vec3::new(200.0 * x as f32, 200.0, 200.0 * y as f32),
                            scale: Vec3::new(7.0, 7.0, 7.0),
                            rotation: Quat::from_scaled_axis(Vec3::new(0.0, 0.0, 0.0)),
                        },
                        SceneRoot(
                            asset_server
                                .load(GltfAssetLabel::Scene(0).from_asset("models/scraper.glb")),
                        ),
                        RigidBody::Dynamic,
                        LockedAxes::ROTATION_LOCKED,
                    ))
                    .with_child((
                        Collider::cuboid(5.0, 12.0, 5.0),
                        Transform::from_translation(Vec3::new(0.0, 13.0, 0.0)),
                    ));
            }
        }
    }
}
