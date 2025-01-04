use crate::utils::*;
use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

pub fn player_shooting(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Velocity), With<components::Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (transform, mut vel) in player_query.iter_mut() {
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        // let right = Vec3::new(local_z.z, 0., -local_z.x);
        let mut projectile_transform = *transform;
        projectile_transform.translation += forward * 100.0;
        if buttons.just_pressed(MouseButton::Left) {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(5.0, 5.0, 10.0)))),
                MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 127))),
                // Transform::from(transform),
                RigidBody::Dynamic,
                projectile_transform,
                // LockedAxes::ROTATION_LOCKED
                LockedAxes::ROTATION_LOCKED,
                Collider::cuboid(5.0, 5.0, 10.0),
                Velocity {
                    linvel: forward * time.delta_secs() * 200000.0,
                    angvel: Vec3::ZERO,
                },
            ));
        }
        // for key in keyboard_input.get_pressed() {
        //     match *key {
        //         KeyCode::KeyW => velocity += forward,
        //         KeyCode::KeyS => velocity -= forward,
        //         KeyCode::KeyA => velocity -= right,
        //         KeyCode::KeyD => velocity += right,
        //         _ => {
        //             return; // quit the execution if no key has been pressed pressed
        //         }
        //     }
        // }

        // velocity = velocity.normalize_or_zero();
        // vel.linvel = velocity * time.delta_secs() * 100000.0;
    }
    // adjust direction with speed and time passed since the last run
    // for mut transform in player_query.iter_mut() {
    //     transform.translation += velocity * time.delta_secs() * 3000.0;
    // }
}
