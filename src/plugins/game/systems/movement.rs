use crate::utils::*;
use bevy::{input::mouse::MouseMotion, math::NormedVectorSpace, prelude::*};
use bevy_window::{CursorGrabMode, PrimaryWindow};

pub fn keyboard_movement(
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
                return; // quit the execution if no key has been pressed pressed
            }
        }
    }

    // adjust direction with speed and time passed since the last run
    movement_vec *= 300.0 * time.delta_seconds();

    for (mut transform) in player_query.iter_mut() {
        movement_vec.y = transform.translation.y; // DO NOT apply to y -> otherwise it will turn to 0
        transform.translation += movement_vec;
    }
}

// majority copied from: https://bevyengine.org/examples/camera/first-person-view-model/
pub fn camera_movement(
    mut player_query: Query<(&mut Transform), With<components::Player>>,
    mut mouse_motion: EventReader<MouseMotion>,
    _time: Res<Time>,
) {
    let mut transform = player_query.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        transform.rotate_y(yaw);
        transform.rotate_local_x(pitch);
    }
}

pub fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor_grabbed: ResMut<resources::GrabbedCursor>,
) {
    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    // primary_window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor.visible = false;

    cursor_grabbed.0 = true;
}

pub fn cursor_ungrab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor_grabbed: ResMut<resources::GrabbedCursor>,
) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;

    cursor_grabbed.0 = false;
}
