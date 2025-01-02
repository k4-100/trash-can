use crate::utils::*;

use bevy::{input::mouse::MouseMotion, math::NormedVectorSpace, prelude::*};

use bevy_rapier3d::prelude::*;
use bevy_window::{CursorGrabMode, PrimaryWindow};

// pub fn keyboard_movement(
//     mut player_query: Query<(&mut Transform), With<components::Player>>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let mut velocity = Vec3::ZERO;
//     for transform in player_query.iter() {
//         let local_z = transform.local_z();
//         let forward = -Vec3::new(local_z.x, 0., local_z.z);
//         let right = Vec3::new(local_z.z, 0., -local_z.x);
//
//         for key in keyboard_input.get_pressed() {
//             match *key {
//                 KeyCode::KeyW => velocity += forward,
//                 KeyCode::KeyS => velocity -= forward,
//                 KeyCode::KeyA => velocity -= right,
//                 KeyCode::KeyD => velocity += right,
//                 _ => {
//                     return; // quit the execution if no key has been pressed pressed
//                 }
//             }
//         }
//     }
//     velocity = velocity.normalize_or_zero();
//     // adjust direction with speed and time passed since the last run
//     for mut transform in player_query.iter_mut() {
//         transform.translation += velocity * time.delta_secs() * 3000.0;
//     }
// }

pub fn keyboard_movement(
    mut player_query: Query<(&Transform, &mut Velocity), With<components::Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut velocity = Vec3::ZERO;
    for (transform, mut vel) in player_query.iter_mut() {
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keyboard_input.get_pressed() {
            match *key {
                KeyCode::KeyW => velocity += forward,
                KeyCode::KeyS => velocity -= forward,
                KeyCode::KeyA => velocity -= right,
                KeyCode::KeyD => velocity += right,
                _ => {
                    return; // quit the execution if no key has been pressed pressed
                }
            }
        }

        velocity = velocity.normalize_or_zero();
        vel.linvel = velocity * time.delta_secs() * 100000.0;
    }
    // adjust direction with speed and time passed since the last run
    // for mut transform in player_query.iter_mut() {
    //     transform.translation += velocity * time.delta_secs() * 3000.0;
    // }
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
        // let pitch = -motion.delta.y * 0.002;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        transform.rotate_y(yaw);
        // transform.rotate_local_x(pitch);
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
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor_options.visible = false;

    cursor_grabbed.0 = true;
}

pub fn cursor_ungrab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor_grabbed: ResMut<resources::GrabbedCursor>,
) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor_options.grab_mode = CursorGrabMode::None;
    primary_window.cursor_options.visible = true;

    cursor_grabbed.0 = false;
}
