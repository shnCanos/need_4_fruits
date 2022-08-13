use bevy::prelude::*;
use crate::PLAYER_GRAVITY;


//region Movement
#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Default for Velocity {
    fn default() -> Self {
        Velocity { x: 0.0, y: 0.0 }
    }
}

#[derive(Component)]
pub struct GravityAffects {
    pub strength: f32,
    pub dashing: bool, // When the player is dashing, gravity doesn't affect it
    pub is_player: bool, // Player gravity works differently
}
impl Default for GravityAffects {
    fn default() -> Self {
        GravityAffects { strength: PLAYER_GRAVITY, dashing: false, is_player: true }
    }
}
//endregion

//region Others
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Aim;
//endregion

//region IsOnWall
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Walls {
    Left,
    Right,
    Floor,
    Roof,

    // This is to make sure the jows values are not overwritten when the player leaves a wall (Player only)
    JustLeft,
}
#[derive(Debug, Component)]
pub struct IsOnWall(pub Option<Walls>);
//endregion