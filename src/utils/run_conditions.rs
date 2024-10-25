use bevy::prelude::*;

use super::resources;

pub fn if_rmb_pressed(buttons: Res<ButtonInput<MouseButton>>) -> bool {
    buttons.pressed(MouseButton::Right)
}

pub fn if_rmb_not_pressed(buttons: Res<ButtonInput<MouseButton>>) -> bool {
    !buttons.pressed(MouseButton::Right)
}

pub fn if_cursor_grabbed(cursor_grabbed: Res<resources::GrabbedCursor>) -> bool {
    cursor_grabbed.0
}
