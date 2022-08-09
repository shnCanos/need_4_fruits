// THIS FILE IS A BACKUP OF THE OLD, BUGGY AND BOILERPLATE PLAYERPLUGIN

use bevy::prelude::*;
use crate::{MAX_PLAYER_JUMPS_MIDAIR, NOT_WALKING_SPEED_ATTRITION, PLAYER_GRAVITY, PLAYER_GRAVITY_FAST_FALLING_SPEED, PLAYER_GRAVITY_ON_WALL, PLAYER_HORIZONTAL_JUMP_WALL, PLAYER_JUMP, PLAYER_SCALE, PLAYER_SIZE, PLAYER_SPEED, PLAYER_VERTICAL_JUMP_WALL, TexturesHandles};
use crate::common_components::{GravityAffects, Velocity};
use crate::controls::Movement;

//region Plugin boilerplate
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
            .add_system(player_movement_system)
            .insert_resource(IsOnWall(None))
            .add_system(player_corners_system)
            .add_system(player_gravity_system);
    }
}
//endregion

//region Player Only Components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
/// This is created in order to fix a bug where the player's speed is overwritten by other tinkering.
/// This is an additional speed, added on top of the player's normal speed.
/// The [`Movement`] struct is ONLY for the controls.
pub struct NotWalkingSpeed {
    x: f32,
    y: f32
}
impl Default for NotWalkingSpeed {
    fn default() -> Self {
        NotWalkingSpeed {
            x: 0.0,
            y: 0.0
        }
    }
}
//endregion

//region Player Only Resources
#[derive(Debug)]
pub enum Walls {
    Left,
    Right,
    Floor,
    Roof,
}
#[derive(Debug)]
pub struct IsOnWall(pub Option<Walls>);
//endregion

fn spawn_player_system(
    mut commands: Commands,
    textures: Res<TexturesHandles>,
) {

    commands.spawn_bundle(
        SpriteBundle {
            texture: textures.ninja.clone(),
            transform: Transform {
                scale: PLAYER_SCALE,
                ..Default::default()
            },
            ..Default::default()
        }
    )
        .insert(Player)
        .insert(Velocity::default())
        .insert(GravityAffects::default())
        .insert(NotWalkingSpeed::default());
}

fn player_movement_system(
    mut movement: ResMut<Movement>,
    mut query: Query<(&mut Velocity, &mut NotWalkingSpeed), With<Player>>,
    wall: Res<IsOnWall>,
) {
    for (mut velocity, mut nws) in query.iter_mut() {
        if wall.0.is_some() {
            movement.lock_x = true;
        } else {
            movement.lock_x = false;
        }

        velocity.x = 0.;
        if !movement.lock_x {
            velocity.x = movement.x * PLAYER_SPEED
        }

        //region Jumps
        // If the player is not on a wall and jumps
        if wall.0.is_none() && movement.jump && movement.jumped < MAX_PLAYER_JUMPS_MIDAIR {
            velocity.y = PLAYER_JUMP;
            movement.jumped += 1;
            movement.is_fast_falling = false;
            movement.jump = false;
            movement.dashed = 0;
        }

        // If the player is on a wall and jumps
        if let Some(side) = &wall.0 {
            if movement.jump && (matches!(*side, Walls::Right) || matches!(*side, Walls::Left)) {
                velocity.y = PLAYER_JUMP + PLAYER_VERTICAL_JUMP_WALL;
                dbg!(&velocity.y);
                movement.is_fast_falling = false;
                nws.x += match *side {
                    Walls::Left => PLAYER_HORIZONTAL_JUMP_WALL,
                    Walls::Right => -PLAYER_HORIZONTAL_JUMP_WALL,
                    _ => panic!(""),
                };
                movement.jumped = 0;
                movement.jump = false;
                movement.dashed = 0;
            }
        }


        //region Apply attrition
        if nws.x > 0. {
            nws.x -= NOT_WALKING_SPEED_ATTRITION;
        }
        if nws.x < 0. {
            nws.x += NOT_WALKING_SPEED_ATTRITION;
        }
        if nws.x < NOT_WALKING_SPEED_ATTRITION && nws.x > NOT_WALKING_SPEED_ATTRITION {
            nws.x = 0.;
        }

        if nws.y > 0. {
            nws.y -= NOT_WALKING_SPEED_ATTRITION;
        }
        if nws.y < NOT_WALKING_SPEED_ATTRITION {
            nws.y = 0.;
        }
        //endregion

        // dbg!(&nws.x);

        velocity.x += nws.x;
        //endregion


    }
}

fn player_corners_system(
    mut query: Query<&mut Transform, With<Player>>,
    window: Res<Windows>,
    mut wall: ResMut<IsOnWall>
) {
    for mut tf in query.iter_mut() {
        let window = window.get_primary().unwrap();
        let max_w = window.width() / 2. - PLAYER_SIZE.x / 2.;
        let max_h = window.height() / 2. - PLAYER_SIZE.y / 2.;
        let mut translation = &mut tf.translation;



        if translation.y <= -max_h {
            // die();
            translation.y = -max_h;
        } else if translation.x <= -max_w {
            translation.x = -max_w;
            wall.0 = Some(Walls::Left);
        } else if translation.y >= max_h {
            translation.y = max_h;
            wall.0 = Some(Walls::Roof);
        } else if translation.x >= max_w {
            translation.x = max_w;
            wall.0 = Some(Walls::Right);
        }
        else {
            wall.0 = None;
        }

        // dbg!(&wall);
    }
}

fn player_gravity_system(
    mut query: Query<(&mut Velocity, &GravityAffects), With<Player>>,
    movement: Res<Movement>,
    wall: Res<IsOnWall>
) {

    for (mut vl, ga) in query.iter_mut()  {
        if movement.is_fast_falling && !ga.dashing && wall.0.is_none(){
            vl.y = PLAYER_GRAVITY_FAST_FALLING_SPEED;
        }
        if !movement.is_fast_falling && !ga.dashing && wall.0.is_none() {
            vl.y -= PLAYER_GRAVITY;
        }
        if wall.0.is_some() {
            vl.y = PLAYER_GRAVITY_ON_WALL;
        }
    }
}

fn die() { todo!() }