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
