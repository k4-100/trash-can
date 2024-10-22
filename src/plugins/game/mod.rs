use bevy::prelude::*;

pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins()
            .add_systems(
                Startup,
                (systems::setup::setup_camera, systems::setup::setup_square),
            );
    }
}
