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

