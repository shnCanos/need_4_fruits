use bevy::prelude::*;

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
}
//endregion

//region Others
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Aim;
//endregion

#[derive(Component)]
pub struct TimeAnimation {
    pub callback: fn(&mut Transform, Vec<f32>, f32),
    pub data : Vec<f32>,
    pub time: f32,
}

impl TimeAnimation {
    pub fn from_callback(callback: fn(&mut Transform, Vec<f32>, f32)) -> Self {
        TimeAnimation { callback, data: vec![], time: 0. }
    }
}

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
