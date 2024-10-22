use bevy::prelude::*;

pub mod plugins;
pub mod utils;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::game::GamePlugin))
        // .add_systems(Startup, ())
        // .add_systems(Update, ())
        .run();
}
