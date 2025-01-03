use bevy::prelude::*;

pub mod systems;
use crate::utils::*;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins()
            .add_systems(
                Startup,
                (
                    setup::setup_camera,
                    // setup::setup_block
                ),
            )
            .add_systems(PostStartup, setup::setup_block_from_txt)
            .add_systems(
                Update,
                (
                    update::update_minimap_position,
                    update::update_coords_text,
                    // movement::keyboard_movement,
                    movement::keyboard_movement,
                    movement::cursor_grab.run_if(run_conditions::if_rmb_pressed),
                    movement::cursor_ungrab.run_if(run_conditions::if_rmb_not_pressed),
                    movement::camera_movement.run_if(run_conditions::if_cursor_grabbed),
                    // movement::camera_movement,
                ),
            )
            .insert_resource(resources::GrabbedCursor(false));
    }
}
