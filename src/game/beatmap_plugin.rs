use crate::GameStates;
use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl};
use std::{collections::HashMap, time::Duration};

use super::{
    osu_reader::{self, OsuFileSection},
    BEATMAP_FILE_NAME, BEATMAP_INITIAL_WAIT_TIME, BEATMAP_MUSIC_OFFSET_TIME,
};

pub struct BeatmapPlugin;

impl Plugin for BeatmapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::Loading).with_system(init_system))
            .add_system_set(
                SystemSet::on_update(GameStates::Game)
                    .with_system(beatmap_start_system)
                    .with_system(background_scaling_system),
            )
            .add_audio_channel::<MusicChannel>();
    }
}

// region Beatmap Resources
#[derive(Default)]
pub struct Beatmap(pub HashMap<String, OsuFileSection>);

#[derive(Default)]
pub struct BeatmapPlayback {
    pub start_timer: Timer,
    pub play_timer: Timer,
    pub music_offset_timer: Timer,
    pub beatmap_started: bool,
    pub current_hit_object_id: usize,
}

// Audio Channel type for Music playback
// Using a custom Audio Channel allows to pause/stop specific audios, while letting others be
pub struct MusicChannel;

#[derive(Component)]
pub struct BackgroundSprite;

// endregion
fn beatmap_start_system(
    mut beatmap_playback: ResMut<BeatmapPlayback>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    beatmap: ResMut<Beatmap>,
    music_channel: Res<AudioChannel<MusicChannel>>,
) {
    // If the timer for starting the fruit spawning has just finished
    if beatmap_playback
        .start_timer
        .tick(time.delta())
        .just_finished()
    {
        // Set the playback to start spawning fruits
        beatmap_playback.beatmap_started = true;
        beatmap_playback.play_timer.unpause();
        // Music should start a short duration after the fruits start spawning
        beatmap_playback.music_offset_timer.unpause();
    }

    // If the timer for starting the music has just finished
    if beatmap_playback
        .music_offset_timer
        .tick(time.delta())
        .just_finished()
    {
        // Try to get the filename for the music to play
        let mut audio_filename = &String::new();
        if let OsuFileSection::KeyValueMap(section_data) = beatmap.0.get("[General]").unwrap() {
            audio_filename = section_data.get("AudioFilename").unwrap();
        }

        // TODO: get rid of temporary forced conversion and actually accept .mp3 files
        let audio_filename = audio_filename.replace(".mp3", ".ogg");
        let music = asset_server.load(&("beatmaps/".to_string() + &audio_filename));

        // Play this 'music' asset in the MusicChannel
        music_channel.play(music);
    }
}

fn init_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Request a restart at the start of the game
    let path = "assets/beatmaps/".to_string() + BEATMAP_FILE_NAME;
    let beatmap = Beatmap(osu_reader::open_osu(&path));
    let mut beatmap_playback = BeatmapPlayback::default();

    // Get the HitObjects list
    if let OsuFileSection::HitObjects(hit_objects) = beatmap.0.get("[HitObjects]").unwrap() {
        // Initialize the timer for the entire beat playing (automatically ends after last HitObject)
        beatmap_playback.play_timer = Timer::new(
            Duration::from_millis(hit_objects.last().map_or(0, |hit_obj| hit_obj.time.into())),
            false,
        );

        // Initialize the timers (probably will want to adjust these timings based on the beatmaps/settings)
        beatmap_playback.start_timer = Timer::from_seconds(BEATMAP_INITIAL_WAIT_TIME, false);
        beatmap_playback.music_offset_timer = Timer::from_seconds(BEATMAP_MUSIC_OFFSET_TIME, false);
    }

    if let OsuFileSection::Events(background) = beatmap.0.get("[Events]").unwrap() {
        let path = "beatmaps/".to_string() + background;

        // Spawn background (starts off invisible, becomes visible when the image is loaded)
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(0., 0., -1.),
                visibility: Visibility { is_visible: false },
                texture: asset_server.load(&path).clone(),
                sprite: Sprite {
                    color: Color::DARK_GRAY,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(BackgroundSprite);
    }

    commands.insert_resource(beatmap);
    commands.insert_resource(beatmap_playback);
}

fn background_scaling_system(
    window: Res<Windows>,
    images: ResMut<Assets<Image>>,
    mut query: Query<(&Handle<Image>, &mut Sprite, &mut Visibility), With<BackgroundSprite>>,
) {
    let (handle, mut sprite, mut visibility) = query.single_mut();

    // If the asset for the Background image has been loaded
    if let Some(image) = images.get(&handle) {
        let window = window.get_primary().unwrap();
        // Set the size of the background to cover the entire screen
        sprite.custom_size = Some(
            image.size() * (window.width() / image.size().x).max(window.height() / image.size().y),
        );

        visibility.is_visible = true;
    }
}
