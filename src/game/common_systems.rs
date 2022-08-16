use crate::game::{
    common_components::{GravityAffects, TimeAnimation, Velocity},
    controls::{Dash, Movement},
    fruit_plugin::{Fruit, FruitPart},
    player_plugin::Player,
    Score, PLAYER_SIZE,
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use super::beatmap_plugin::{BeatmapPlayback, MusicChannel};

pub struct CommonSystems;

impl Plugin for CommonSystems {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(move_with_velocity_system)
                .with_system(gravity_system)
                .with_system(process_time_animations)
                .with_system(restart_game_system),
        )
        .add_event::<RestartEvent>()
        .add_startup_system(init_system);
    }
}

fn process_time_animations(
    mut query: Query<(&mut Transform, &mut TimeAnimation)>,
    time: Res<Time>,
) {
    query.for_each_mut(|(mut tf, mut time_animation)| {
        time_animation.time += time.delta_seconds() as f32;
        (time_animation.callback)(&mut tf, time_animation.data.clone(), time_animation.time);
    });
}

fn init_system(mut restart_events: EventWriter<RestartEvent>) {
    // Request a restart at the start of the game
    restart_events.send_default()
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

#[derive(Default)]
pub struct RestartEvent;

fn restart_game_system(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    despawn_fruit_query: Query<Entity, Or<(With<Fruit>, With<FruitPart>)>>,
    mut movement: ResMut<Movement>,
    mut dash: ResMut<Dash>,
    mut beatmap_playback: ResMut<BeatmapPlayback>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    window: Res<Windows>,
    restart_events: EventReader<RestartEvent>,
) {
    if restart_events.is_empty() {
        return;
    }
    // Prevent events from staying active next frame.
    restart_events.clear();

    for (mut tf, mut vl) in query.iter_mut() {
        let window = window.get_primary().unwrap();
        let max_w = window.width() / 2. - PLAYER_SIZE.x / 2.;

        // Reset variables
        score.0 = 0;
        
        vl.x = 0.;
        vl.y = 0.;
        tf.translation.x = -max_w;
        tf.translation.y = 0.;
        
        movement.jump = false;
        movement.jumped = 0;
        movement.is_fast_falling = false;
        
        dash.dashed = 0;
        dash.is_dashing = false;
        dash.trying_to_dash = false;
        dash.direction = Vec2::ZERO;
        
        music_channel.stop();
        
        beatmap_playback.current_hit_object_id = 0;
        beatmap_playback.play_timer.reset();
        beatmap_playback.play_timer.pause();
        beatmap_playback.music_offset_timer.reset();
        beatmap_playback.music_offset_timer.pause();
        beatmap_playback.start_timer.reset();
        beatmap_playback.beatmap_started = false;
        // TODO: wait does this really not work?
        // *movement.as_mut() = Movement::default();
        // *dash.as_mut() = Dash::default();

        // Despawn all fruits
        despawn_fruit_query.for_each(|entity| commands.entity(entity).despawn());

        break; // Tell the compiler that the loop won't repeat more than once
    }
}
