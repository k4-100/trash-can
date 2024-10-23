use bevy::prelude::*;

pub mod systems;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins()
            .add_systems(Startup, (setup::setup_camera, setup::setup_block))
            .add_systems(Update, (movement::keyboard_move));
    }
}
