use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_render::camera::Viewport;

use crate::utils::components;

pub fn update_minimap_window_position(
    window: Query<&Window>,
    mut minimap_camera: Query<&mut Camera, With<components::MiniMapCamera>>,
    mut player_camera: Query<
        &mut Camera,
        (
            With<components::CurrentPlayer>,
            Without<components::MiniMapCamera>,
        ),
    >,
) {
    let window = window.single();

    let physical_size = window.physical_size();

    let mut mnmp_camera = minimap_camera.single_mut();
    let mut mnmp_camera_viewport = mnmp_camera.viewport.clone().unwrap();
    mnmp_camera_viewport = Viewport {
        physical_position: UVec2 {
            x: physical_size.x - mnmp_camera_viewport.physical_size.x,
            y: physical_size.y - mnmp_camera_viewport.physical_size.y,
        },
        ..mnmp_camera_viewport
    };
    mnmp_camera.viewport = Some(mnmp_camera_viewport);
    // println!("{}", player_camera.iter().count());

    let mut plr_camera = player_camera.single_mut();
    let mut plr_camera_viewport = plr_camera.viewport.clone().unwrap();
    plr_camera_viewport = Viewport {
        physical_size: UVec2 {
            x: physical_size.x,
            y: physical_size.y,
        },
        ..plr_camera_viewport
    };
    plr_camera.viewport = Some(plr_camera_viewport);
}

pub fn update_minimap_global_position(
    player_camera_query: Query<
        &Transform,
        (
            With<components::CurrentPlayer>,
            Without<components::MiniMapCamera>,
        ),
    >,
    mut minimap_camera_query: Query<&mut Transform, With<components::MiniMapCamera>>,
) {
    let player_camera_transform = player_camera_query.single();
    let mut minimap_camera_transform = minimap_camera_query.single_mut();
    let mut new_translation = player_camera_transform.translation;
    new_translation.y = minimap_camera_transform.translation.y;

    minimap_camera_transform.translation = new_translation;
}

pub fn update_coords_text(
    player_transform_query: Query<&Transform, With<components::CurrentPlayer>>,
    mut text_query: Query<&mut Text, With<components::CoordsText>>,
) {
    // let position = player_transform_query.single().translation;
    // let mut text = text_query.single_mut();
    // *text = Text::new(format!(" x: {}\ny: {}", position.x, position.z));
}
