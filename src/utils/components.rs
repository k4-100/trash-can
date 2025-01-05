use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CurrentPlayer;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct MiniMapCamera;

#[derive(Component, Debug)]
pub struct CoordsText;

#[derive(Component, Debug, Default)]
pub enum Mob {
    Hostile,
    #[default]
    Passive,
}
