use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_render::camera::Viewport;

use crate::utils::components;

pub fn update_minimap_position(
    window: Query<&Window>,
    mut minimap_camera: Query<&mut Camera, With<components::MiniMapCamera>>,
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
}

pub fn update_coords_text(
    player_transform_query: Query<&Transform, With<components::CurrentPlayer>>,
    mut text_query: Query<&mut Text, With<components::CoordsText>>,
) {
    let position = player_transform_query.single().translation;
    let mut text = text_query.single_mut();
    *text = Text::new(format!(" x: {}\ny: {}", position.x, position.z));
}
