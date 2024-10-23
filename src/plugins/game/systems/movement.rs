use std::iter;

use crate::utils::*;
use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub fn keyboard_move(
    mut player_query: Query<(&mut Transform), With<components::Player>>,
    // keys: KeyboardInput,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut movement_vec = Vec3::ZERO;

    for key in keyboard_input.get_pressed() {
        match *key {
            KeyCode::KeyW => movement_vec.z -= 1.0,
            KeyCode::KeyS => movement_vec.z += 1.0,
            KeyCode::KeyA => movement_vec.x -= 1.0,
            KeyCode::KeyD => movement_vec.x += 1.0,
            _ => {
                return;
            }
        }
    }

    movement_vec *= 300.0 * time.delta_seconds();

    for (mut transform) in player_query.iter_mut() {
        movement_vec.y = transform.translation.y;
        transform.translation += movement_vec;
    }
}
