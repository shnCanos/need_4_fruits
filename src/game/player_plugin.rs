use crate::game::common_components::{IsOnWall, TimeAnimation, Velocity, Walls};
use crate::game::common_systems::RestartEvent;
use crate::game::controls::{Dash, Movement};
use crate::game::fruit_plugin::CutAffects;
use crate::game::{
    TexturesHandles, DASH_DURATION, DASH_SPEED, FRUITS_SIZE, JUMP_OFF_WALL_SPEED_ATTRITION,
    MAX_PLAYER_DASHES_MIDAIR, MAX_PLAYER_JUMPS_MIDAIR, PLAYER_FAST_FALLING_SPEED, PLAYER_GRAVITY,
    PLAYER_GRAVITY_ON_WALL, PLAYER_HORIZONTAL_JUMP_WALL, PLAYER_JUMP, PLAYER_SCALE, PLAYER_SIZE,
    PLAYER_SPEED, PLAYER_VERTICAL_JUMP_WALL,
};
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

//region Plugin boilerplate
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
            .add_system(player_corners_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(movement_air_criteria)
                    .with_system(player_movement_air_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(movement_wall_criteria)
                    .with_system(player_movement_wall_system),
            )
            .add_system(can_dash_system)
            .add_system(dash_system)
            .add_system(dash_aura_system)
            .add_system(fruit_collision_system)
            .add_system(player_bottom_system);
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
    y: f32,
}

impl JumpOffWallSpeed {
    // BTW jows stands for this struct's name
    fn apply_friction(&mut self) {
        let friction = |x: f32| (x.abs() - JUMP_OFF_WALL_SPEED_ATTRITION).max(0.) * x.signum();

        self.x = friction(self.x);
        self.y = friction(self.y);
    }

    fn reset(&mut self) {
        self.x = 0.;
        self.y = 0.;
    }
}

impl Default for JumpOffWallSpeed {
    fn default() -> Self {
        JumpOffWallSpeed { x: 0.0, y: 0.0 }
    }
}

#[derive(Component)]
pub struct DashAura;
//endregion

//region Player Only Resources

//endregion

fn spawn_player_system(mut commands: Commands, textures: Res<TexturesHandles>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: PLAYER_SCALE,
                ..Default::default()
            },
            texture: textures.ninja.clone(),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity::default())
        .insert(IsOnWall(None))
        .insert(JumpOffWallSpeed::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: textures.aura.clone(),
                    ..Default::default()
                })
                .insert(TimeAnimation::from_callback(|tf, _, t| {
                    tf.rotation = Quat::from_rotation_z(t * 3.);
                    tf.scale = Vec3::splat(0.9 + (t * 2.5).sin() * 0.1);
                }))
                .insert(DashAura);
        });
}

fn player_corners_system(
    mut query: Query<(&mut Transform, &mut IsOnWall, &mut Velocity), With<Player>>,
    window: Res<Windows>,
) {
    for (mut tf, mut wall, mut velocity) in query.iter_mut() {
        // Make sure this doesn't start registering the wall right after the player left
        // Or some nasty bugs happen
        if !matches!(wall.0, Some(Walls::JustLeft)) {
            let window = window.get_primary().unwrap();
            let max_w = window.width() / 2. - PLAYER_SIZE.x / 2.;
            let min_h = -(window.height() / 2. + PLAYER_SIZE.y);
            let max_h = window.height() / 2. - PLAYER_SIZE.y / 2.;
            let mut translation = &mut tf.translation;

            if translation.x >= max_w {
                translation.x = max_w;
                wall.0 = Some(Walls::Right);
            } else if translation.x <= -max_w {
                translation.x = -max_w;
                wall.0 = Some(Walls::Left);
            } else {
                wall.0 = None;
            }

            if translation.y <= min_h {
                wall.0 = Some(Walls::Floor);
            } else if translation.y > max_h {
                translation.y = max_h;
                wall.0 = Some(Walls::Roof);
                velocity.y = 0.;
            }
        }
    }
}

fn movement_air_criteria(wall: Query<&IsOnWall, With<Player>>, dash: Res<Dash>) -> ShouldRun {
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

fn player_movement_air_system(
    mut query: Query<(&mut Velocity, &mut JumpOffWallSpeed, &mut IsOnWall, &mut Sprite), With<Player>>,
    mut movement: ResMut<Movement>,
) {
    for (mut velocity, mut jows, mut wall, mut sprite) in query.iter_mut() {
        // Make sure the player keeps the momentum until it is actually in the air
        if let Some(Walls::JustLeft) = wall.0 {
            wall.0 = None;
        }

        velocity.x = movement.x * PLAYER_SPEED;

        if movement.x > 0. {
            sprite.flip_x = false;
        } else if movement.x < 0. {
            sprite.flip_x = true;
        }

        if movement.jump {
            movement.jump = false;

            if movement.jumped < MAX_PLAYER_JUMPS_MIDAIR {
                //region Change movement variables
                movement.jumped += 1;
                movement.is_fast_falling = false;
                //endregion

                velocity.y = PLAYER_JUMP;
            }
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

        jows.apply_friction();
        //endregion
    }
}

fn movement_wall_criteria(wall: Query<&IsOnWall, With<Player>>, dash: Res<Dash>) -> ShouldRun {
    if dash.is_dashing {
        return ShouldRun::No;
    }

    match movement_air_criteria(wall, dash) {
        ShouldRun::No => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

fn player_movement_wall_system(
    mut query: Query<(&mut Velocity, &mut JumpOffWallSpeed, &mut IsOnWall, &mut Sprite), With<Player>>,
    mut movement: ResMut<Movement>,
    mut dash: ResMut<Dash>,
) {
    for (mut velocity, mut jows, mut wall, mut sprite) in query.iter_mut() {
        // If the player is on a wall, don't dash
        // And ignore the trying to dash or the player
        // Will dash right after leaving the wall
        if let Some(wall) = &wall.0 {
            if !matches!(wall, &Walls::Roof) {
                dash.is_dashing = false;
                dash.trying_to_dash = false;
                dash.direction = Vec2::default();

                // Restart the dashes count
                dash.dashed = 0;
            }
        }

        if !matches!(wall.0, Some(Walls::JustLeft)) {
            velocity.x = 0.;

            velocity.y = -PLAYER_GRAVITY_ON_WALL;

            if matches!(wall.0, Some(Walls::Left)) {
                sprite.flip_x = false;
            } else if matches!(wall.0, Some(Walls::Right)) {
                sprite.flip_x = true;
            }

            // There may be some jows left from the other wall if you travel fast enough
            // From one side to the other
            if !movement.jump {
                jows.reset()
            }

            if movement.jump {
                let signal = match &wall.0 {
                    Some(Walls::Left) => 1.,
                    Some(Walls::Right) => -1.,
                    _ => 0.,
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
            jows.apply_friction();

            velocity.x = jows.x;
            velocity.y = jows.y;
        }
    }
}

fn can_dash_system(mut dash: ResMut<Dash>) {
    if !dash.trying_to_dash || dash.is_dashing {
        return; // Do nothing
    }

    if dash.dashed >= MAX_PLAYER_DASHES_MIDAIR {
        dash.trying_to_dash = false;
        return; // Do nothing
    }

    dash.is_dashing = true;
    dash.duration = Timer::from_seconds(DASH_DURATION, false);
    dash.dashed += 1;
}

fn dash_system(
    mut query: Query<(&mut Velocity, &mut JumpOffWallSpeed, &mut Sprite), With<Player>>,
    mut dash: ResMut<Dash>,
    time: Res<Time>,
    movement: Res<Movement>,
) {
    if !dash.is_dashing {
        return; // Do nothing
    }

    let end_dash = |mut dash: ResMut<Dash>| {
        // Needs to specify dash or two mutable borrows may occur at the same time
        //region Change dash variables
        dash.direction = Vec2::default();
        dash.is_dashing = false;
        dash.trying_to_dash = false;
        //endregion
    };

    // Cancel the dash if the player jumps
    if movement.jump && movement.jumped < MAX_PLAYER_JUMPS_MIDAIR {
        end_dash(dash);
        return;
    }

    for (mut velocity, mut jows, mut sprite) in query.iter_mut() {
        if dash.duration.finished() {
            end_dash(dash);

            // Also return velocity to zero
            // Or some glitches happen
            // When you dash upwards
            velocity.x = 0.;
            velocity.y = 0.;
            return;
        }

        if dash.direction.x > 0. {
            sprite.flip_x = false;
        } else if dash.direction.x < 0. {
            sprite.flip_x = true;
        }

        let dash_velocity = dash.direction.normalize() * DASH_SPEED;
        (velocity.x, velocity.y) = (dash_velocity.x, dash_velocity.y);

        jows.reset();

        dash.apply_time(&time);
    }
}

fn fruit_collision_system(
    mut dash: ResMut<Dash>,
    mut movement: ResMut<Movement>,
    mut fruit_query: Query<(&Transform, &mut CutAffects)>,
    player_query: Query<&Transform, With<Player>>,
) {
    // The fruits are only cut when the player is dashing
    if !dash.is_dashing {
        return;
    }

    for player_tf in player_query.iter() {
        for (fruits_tf, mut cut_affects) in fruit_query.iter_mut() {
            let collision = collide(
                player_tf.translation,
                PLAYER_SIZE,
                fruits_tf.translation,
                FRUITS_SIZE,
            );

            if let Some(_) = collision {
                // Has been cut
                cut_affects.is_cut = true;

                dash.dashed = (dash.dashed as i32 - 1).max(0) as usize;
                movement.jumped = (movement.jumped as i32 - 1).max(0) as usize;
            }
        }
    }
}

fn dash_aura_system(mut query: Query<&mut Visibility, With<DashAura>>, dash: ResMut<Dash>) {
    query.for_each_mut(|mut visibility| {
        visibility.is_visible = dash.dashed < MAX_PLAYER_DASHES_MIDAIR
    });
}

fn player_bottom_system(
    mut query: Query<&IsOnWall, With<Player>>,
    mut restart_events: EventWriter<RestartEvent>,
) {
    for is_on_wall in query.iter_mut() {
        if matches!(is_on_wall.0, Some(Walls::Floor)) {
            // Request game to be restarted
            restart_events.send_default();
        }
    }
}
