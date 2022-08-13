use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use crate::{JUMP_OFF_WALL_SPEED_ATTRITION, MAX_PLAYER_JUMPS_MIDAIR, PLAYER_GRAVITY, PLAYER_FAST_FALLING_SPEED, PLAYER_GRAVITY_ON_WALL, PLAYER_HORIZONTAL_JUMP_WALL, PLAYER_JUMP, PLAYER_SCALE, PLAYER_SIZE, PLAYER_SPEED, PLAYER_VERTICAL_JUMP_WALL, TexturesHandles, MAX_PLAYER_DASHES_MIDAIR, DASH_DURATION, DASH_SPEED};
use crate::common_components::{GravityAffects, Velocity};
use crate::controls::{Dash, Movement, DashDirection};

//region Plugin boilerplate
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
            .add_system(player_corners_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(movement_air_criteria)
                    .with_system(player_movement_air_system)
            )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(movement_wall_criteria)
                .with_system(player_movement_wall_system)
        )
            .add_system(can_dash_system)
            .add_system(dash_system);
    }
}
//endregion

//region Player Only Components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
/// This is created in order to fix a bug where the player's speed is overwritten by other functions when on a wall.
/// This is an additional speed that the player gains when jumping off a wall, added on top of the player's normal speed.
/// The [`Movement`] struct is ONLY for the controls.
pub struct JumpOffWallSpeed {
    x: f32,
    y: f32
}
impl JumpOffWallSpeed {
    // BTW jows stands for this struct's name
    fn jows_attrition(&mut self) {
        let attrition = |x: &mut f32| {
            if x.is_sign_positive() {
                if *x > JUMP_OFF_WALL_SPEED_ATTRITION {
                    *x -= JUMP_OFF_WALL_SPEED_ATTRITION;
                } else {
                    *x = 0.;
                }
            }
            if x.is_sign_negative() {
                if *x < JUMP_OFF_WALL_SPEED_ATTRITION {
                    *x += JUMP_OFF_WALL_SPEED_ATTRITION;
                } else {
                    *x = 0.;
                }
            }
        };

        attrition(&mut self.x);
        attrition(&mut self.y);
    }
    fn zero_the_values ( &mut self ) {
        if self.x != 0. {
            self.x = 0.;
        }
        if self.y != 0. {
            self.y = 0.;
        }
    }
}

impl Default for JumpOffWallSpeed {
    fn default() -> Self {
        JumpOffWallSpeed {
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
    JustLeft, // This is to make sure the jows values are not overwritten when the player leaves a wall
}
#[derive(Debug, Component)]
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
        .insert(IsOnWall(None))
        .insert(JumpOffWallSpeed::default());
}

fn player_corners_system(
    mut query: Query<(&mut Transform, &mut IsOnWall), With<Player>>,
    window: Res<Windows>,
) {
    for (mut tf, mut wall) in query.iter_mut() {

        // Make sure this doesn't start registering the wall right after the player left
        // Or some nasty bugs happen
        if !matches!(wall.0, Some(Walls::JustLeft)) {
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
}

fn movement_air_criteria(
    wall: Query<&IsOnWall, With<Player>>,
    dash: Res<Dash>
) -> ShouldRun {
    if dash.is_dashing {
        return ShouldRun::No;
    }

    if let Some(side) = &wall.get_single().unwrap().0 {
        return match side {
            Walls::Roof => ShouldRun::Yes,
            Walls::JustLeft => ShouldRun::Yes,
            _ => ShouldRun::No,
        };
    }
    ShouldRun::Yes
}

fn player_movement_air_system (
    mut query: Query<(&mut Velocity, &mut JumpOffWallSpeed, &mut IsOnWall), With<Player>>,
    mut movement: ResMut<Movement>,
) {
    for (mut velocity, mut jows, mut wall) in query.iter_mut() {
        // Make sure the player keeps the momentum until it is actually in the air
        if let Some(Walls::JustLeft) = wall.0 {
            wall.0 = None;
        }

        velocity.x = movement.x * PLAYER_SPEED;

        if movement.jump && movement.jumped < MAX_PLAYER_JUMPS_MIDAIR {
            velocity.y = PLAYER_JUMP;

            //region Change movement variables
            movement.jumped += 1;
            movement.jump = false;
            movement.is_fast_falling = false;
            //endregion
        }

        if movement.is_fast_falling {
            velocity.y = PLAYER_FAST_FALLING_SPEED;
        } else {
            // Apply Gravity
            velocity.y -= PLAYER_GRAVITY;
        }

        //region Apply JumpOffWallSpeed
        velocity.x += jows.x;
        velocity.y += jows.y;

        //region Apply attrition to JumpOffWallSpeed
        jows.jows_attrition()
        //endregion

        //endregion
    }

}

fn movement_wall_criteria(
    wall: Query<&IsOnWall, With<Player>>,
    dash: Res<Dash>
) -> ShouldRun {
    if dash.is_dashing {
        return ShouldRun::No;
    }

    match movement_air_criteria(wall, dash) {
        ShouldRun::No => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

fn player_movement_wall_system(
    mut query: Query<(&mut Velocity, &mut JumpOffWallSpeed, &mut IsOnWall), With<Player>>,
    mut movement: ResMut<Movement>,
    mut dash: ResMut<Dash>
) {
    for (mut velocity, mut jows, mut wall) in query.iter_mut() {

        // If the player is on a wall, don't dash
        // And ignore the trying to dash or the player
        // Will dash right after leaving the wall
        if let Some(wall) = &wall.0 {
            if !matches!(wall, &Walls::Roof) {
                dash.is_dashing = false;
                dash.trying_to_dash = false;

                // Restart the dashes count
                dash.dashed = 0;
            }
        }

        if !matches!(wall.0, Some(Walls::JustLeft)) {
            velocity.x = 0.;
            velocity.y = -PLAYER_GRAVITY_ON_WALL;

            // There may be some jows left from the other wall if you travel fast enough
            // From one side to the other
            if !movement.jump {
                jows.zero_the_values()
            }

            if movement.jump {
                let signal = match &wall.0 {
                    Some(Walls::Left) => 1.,
                    Some(Walls::Right) => -1.,
                    _ => unimplemented!(),
                };
                jows.x = PLAYER_HORIZONTAL_JUMP_WALL * signal;
                jows.y = PLAYER_VERTICAL_JUMP_WALL;

                //region Change movement variables
                movement.jump = false;
                movement.jumped = 0;
                movement.is_fast_falling = false;
                //endregion

                // Change wall status
                wall.0 = Some(Walls::JustLeft);

                velocity.x = jows.x;
                velocity.y = jows.y;
            }
        } else {
            jows.jows_attrition();

            velocity.x = jows.x;
            velocity.y = jows.y;
        }
    }
}

fn can_dash_system (
    mut dash: ResMut<Dash>
) {
    if !dash.trying_to_dash || dash.is_dashing {
        return; // Do nothing
    }

    // The logic that stops the player from dashing
    // When on a wall is defined in player_movement_wall_system()
    // Due to some bugs that arose

    if dash.dashed >= MAX_PLAYER_DASHES_MIDAIR {
        dash.trying_to_dash = false;
        return; // Do nothing
    }

    dash.is_dashing = true;
    dash.duration = Timer::from_seconds(DASH_DURATION, false);
}

fn dash_system(
    mut query: Query<(&mut Velocity, &mut JumpOffWallSpeed), With<Player>>,
    mut dash: ResMut<Dash>,
    time: Res<Time>,
) {
    if !dash.is_dashing {
        return; // Do nothing
    }
    for (mut velocity, mut jows) in query.iter_mut() {
        if dash.duration.finished() {
            // Rewrite the dashed variables
            dash.direction = DashDirection::default();
            dash.is_dashing = false;
            dash.trying_to_dash = false;
            dash.dashed += 1;

            // Return velocity to zero
            // Or some glitches happen
            // When you dash upwards
            velocity.x = 0.;
            velocity.y = 0.;
            return;
        }


        velocity.x = dash.direction.x * DASH_SPEED;
        velocity.y = dash.direction.y * DASH_SPEED;

        jows.zero_the_values();

        dash.apply_time(&time);


    }
}