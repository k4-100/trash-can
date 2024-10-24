use bevy::prelude::*;

pub fn if_rmb_pressed(buttons: Res<ButtonInput<MouseButton>>) -> bool {
    return buttons.pressed(MouseButton::Right);
}

pub fn if_rmb_not_pressed(buttons: Res<ButtonInput<MouseButton>>) -> bool {
    return !buttons.pressed(MouseButton::Right);
}
