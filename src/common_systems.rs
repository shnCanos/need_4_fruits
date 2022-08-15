use crate::{common_components::{GravityAffects, TimeAnimation, Velocity, IsOnWall}, player_plugin::Player, Score, fruit_plugin::{Fruit, FruitPart}, controls::{Movement, Dash}};
use bevy::prelude::*;

pub struct CommonSystems;

impl Plugin for CommonSystems {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(move_with_velocity_system)
                .with_system(gravity_system)
                .with_system(process_time_animations)
                .with_system(restart_game_system)
        )
        .insert_resource(RestartGame(false));
    }
}

fn process_time_animations(mut query: Query<(&mut Transform, &mut TimeAnimation)>, time: Res<Time>) {
    query.for_each_mut(|(mut tf, mut time_animation)| {
        time_animation.time += time.delta_seconds() as f32;
        (time_animation.callback)(&mut tf, time_animation.time);
    });
}

fn move_with_velocity_system(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut tf, vl) in query.iter_mut() {
        let mut translation: &mut Vec3 = &mut tf.translation;

        // Multiply by 60 and time delta in order to
        // Make the game independent of frames
        // (My monitor is 60hz so that's the default)
        translation.x += vl.x * 60. * time.delta_seconds();
        translation.y += vl.y * 60. * time.delta_seconds();
    }
}

fn gravity_system(mut query: Query<(&mut Velocity, &GravityAffects)>) {
    for (mut vl, ga) in query.iter_mut() {
        vl.y -= ga.strength
    }
}

pub struct RestartGame( pub bool );
fn restart_game_system(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut score: ResMut<Score>,
    mut commands: Commands, 
    despawn_fruit_query: Query<Entity, Or<(With<Fruit>, With<FruitPart>)>>,
    mut movement: ResMut<Movement>,
    mut dash: ResMut<Dash>,
    mut restart: ResMut<RestartGame>,
) {
    for (mut tf, mut vl) in query.iter_mut() {
        if !restart.0 {
            return;
        }

        // Reset variables
        score.0 = 0;
        vl.x = 0.;
        vl.y = 0.;
        tf.translation.x = 0.;
        tf.translation.y = 0.;
        movement.jumped = 0;
        movement.is_fast_falling = false;
        dash.dashed = 0;
        restart.0 = false;

        // Despawn all fruits
        despawn_fruit_query.for_each(|entity|commands.entity(entity).despawn());


        break; // Tell the compiler that the loop won't repeat more than once
    }
}