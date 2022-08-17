use crate::game::common_components::{GravityAffects, IsOnWall, TimeAnimation, Velocity, Walls};
use crate::game::common_systems::RestartEvent;
use crate::game::{
    Score, TexturesHandles, FRUITS_SCALE, FRUITS_SIZE, FRUIT_SPEED, MAX_FRUIT_PIECE_SPEED,
    NUMBER_OF_FRUIT_PIECES,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::GameStates;

use super::beatmap_plugin::{Beatmap, BeatmapPlayback};
use super::osu_reader::OsuFileSection;
use super::{
    BEATMAP_MUSIC_OFFSET_TIME, EFFECTIVE_SCREEN_WIDTH_PERCENT, FRUITS_GRAVITY_FALL,
    FRUITS_GRAVITY_HOLD, FRUITS_GRAVITY_UP,
};

//region Plugin Boilerplate
pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
            SystemSet::on_update(GameStates::Game)
                .label("corners_and_reach_bottom")
                .with_system(fruit_corners_system)
                .with_system(fruits_reach_bottom_system)
        )

            .add_system_set(
            SystemSet::on_update(GameStates::Game)
                .with_system(spawn_fruit_system).after("corners_and_reach_bottom")
        )
            .add_system_set(
                SystemSet::on_update(GameStates::Game)
                    .with_system(fruits_get_cut_system)
                    .with_system(fruits_cuttable_system)
                    .with_system(fruit_part_eliminate_system)
            );
    }
}
//endregion

//region Fruit Only Components
#[derive(Component)]
pub struct Fruit {
    pub texture_id: usize, // The id of the fruit's texture, chosen randomly
}

#[derive(Component)]
pub struct FruitPart;

#[derive(Component)]
pub struct CutAffects {
    pub is_cut: bool,
    pub can_be_cut: bool,
}
//endregion

fn spawn_fruit_system(
    mut commands: Commands,
    mut beatmap_playback: ResMut<BeatmapPlayback>,
    window: Res<Windows>,
    textures: Res<TexturesHandles>,
    beatmap: Res<Beatmap>,
    time: Res<Time>,
) {
    // Don't spawn any fruits until the beatmap has started
    if !beatmap_playback.beatmap_started {
        return;
    }

    // Get the amount of milliseconds since the beatmap started playing
    let current_millis = (beatmap_playback
        .play_timer
        .tick(time.delta())
        .elapsed_secs()
        * 1000.) as u32;

    // Get the vector of HitObjects from the Beatmap data
    if let Some(section) = beatmap.0.get("[HitObjects]") {
        if let OsuFileSection::HitObjects(hit_objects) = section {
            // Get the current HitObject from the vector of HitObjects
            let hit_object = &hit_objects[beatmap_playback.current_hit_object_id];

            // Return if it's still not time to spawn the fruit
            if current_millis < hit_object.time {
                return;
            }

            beatmap_playback.current_hit_object_id += 1;

            // Random fruit generation
            let number_of_fruits = textures.fruits.len();
            let index_of_fruit = thread_rng().gen_range(0..number_of_fruits);
            let texture = textures.fruits[index_of_fruit].clone();

            // Random position generation
            let window = window.get_primary().unwrap();
            let effective_width = window.width() * EFFECTIVE_SCREEN_WIDTH_PERCENT;
            let y_spawn_position = -window.height() / 2. - 50.;
            let x_spawn_position =
                (hit_object.position.x / 640. * effective_width) - effective_width / 2.;

            // Calculations for the fruit speed (gone sorta wrong)
            // let y_factor = hit_object.position.y / 480. * 5.;
            // let y_peak_position = (0.5 - hit_object.position.y / 480.) * effective_height;
            // let effective_height = window.height() * EFFECTIVE_SCREEN_WIDTH_PERCENT;

            // ut = s - 1/2at^2
            // u = (s - 1/2at^2) / t
            // u = s/t - 1/2at
            // let time_to_peak = 0.7 * 30.;
            // let fruit_speed = (y_peak_position - y_spawn_position) / time_to_peak + 0.5 * FRUITS_GRAVITY_UP * time_to_peak;

            commands
                .spawn_bundle(SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3::new(x_spawn_position, y_spawn_position, 0.0),
                        scale: FRUITS_SCALE,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Velocity {
                    x: thread_rng().gen_range(-0.4..0.4),
                    y: FRUIT_SPEED,
                })
                .insert(GravityAffects {
                    strength: FRUITS_GRAVITY_UP,
                })
                .insert(IsOnWall(None))
                .insert(Fruit {
                    texture_id: index_of_fruit,
                })
                .insert(CutAffects {
                    is_cut: false,
                    can_be_cut: false,
                })
                .insert(TimeAnimation::from_callback(|tf, _, t| {
                    tf.rotation = Quat::from_rotation_z(t * 4.0);
                    tf.scale = FRUITS_SCALE * (1. / BEATMAP_MUSIC_OFFSET_TIME * t).min(1.);
                }));
        }
    }
}

fn fruit_corners_system(
    mut query: Query<(&mut Transform, &mut IsOnWall), Or<(With<Fruit>, With<FruitPart>)>>,
    window: Res<Windows>,
) {
    for (tf, mut is_on_wall) in query.iter_mut() {
        let translation = tf.translation;
        // We add the FRUIT_SIZE to the height because we care about when the fruit leaves the screen
        let max_h = window.get_primary().unwrap().height() / 2. + FRUITS_SIZE.y / 2.;

        if translation.y <= -max_h {
            is_on_wall.0 = Some(Walls::Floor)
        } else {
            is_on_wall.0 = None;
        }

        // Note: With fruits, the only values that matter
        // Are whether the fruits are in the air or below
        // The stage
    }
}

fn fruit_part_eliminate_system(
    mut commands: Commands,
    mut query: Query<(Entity, &IsOnWall), With<FruitPart>>,
) {
    for (entity, wall) in query.iter_mut() {
        // If the fruit part hits the floor
        if let Some(_) = wall.0 {
            // Despawn the part
            commands.entity(entity).despawn();
        }
    }
}

fn fruits_reach_bottom_system(
    mut query: Query<(Entity, &IsOnWall), With<Fruit>>,
    mut restart_events: EventWriter<RestartEvent>,
) {
    for (_, wall) in query.iter_mut() {
        // If the fruit hits the floor
        if let Some(_) = wall.0 {
            // Request game to be restarted
            restart_events.send_default();
            break;
        }
    }
}

fn fruits_get_cut_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &CutAffects, &Fruit)>,
    mut score: ResMut<Score>,
    textures: Res<TexturesHandles>,
) {
    for (entity, transform, cut_affects, fruit) in query.iter() {
        if !cut_affects.is_cut {
            continue;
        }

        score.0 += 1;

        // Spawn as many Fruit Parts as in NUMBER_OF_FRUIT_PIECES
        for part_id in 0..(NUMBER_OF_FRUIT_PIECES as usize) {
            let fruit_atlas = textures.fruits_pieces_texture_atlas[fruit.texture_id].clone();

            let translation = transform.translation;

            let x_vl = thread_rng().gen_range(-MAX_FRUIT_PIECE_SPEED..MAX_FRUIT_PIECE_SPEED);
            let y_vl = thread_rng().gen_range(0.0..MAX_FRUIT_PIECE_SPEED);

            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: fruit_atlas,
                    sprite: TextureAtlasSprite {
                        index: part_id,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation,
                        scale: FRUITS_SCALE * 0.75,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(FruitPart) // It's a part of a fruit
                .insert(Velocity { x: x_vl, y: y_vl }) // The pieces of fruit explode
                .insert(GravityAffects {
                    strength: FRUITS_GRAVITY_FALL * 2.,
                }) // The pieces of the fruit are affected by gravity
                .insert(IsOnWall(None))
                .insert(TimeAnimation {
                    callback: |tf, data, t| {
                        tf.rotation = Quat::from_rotation_z(t * data[0]);
                        tf.scale = FRUITS_SCALE * 0.75 * (1. - 0.5 * t);
                    },
                    data: vec![
                        thread_rng().gen_range(2.0..4.0)
                            * if thread_rng().gen_bool(0.5) { 1. } else { -1. },
                    ],
                    time: 0.,
                }); // We check whether it hit the floor to despawn
        }

        commands.entity(entity).despawn();
    }
}

fn fruits_cuttable_system(
    mut query: Query<(&Velocity, &mut Sprite, &mut CutAffects, &mut GravityAffects), With<Fruit>>,
) {
    for (velocity, mut sprite, mut cut_affects, mut gravity_affects) in query.iter_mut() {
        cut_affects.can_be_cut = velocity.y <= 2.;
        if velocity.y < 0. {
            gravity_affects.strength = FRUITS_GRAVITY_FALL;
        } else if cut_affects.can_be_cut {
            gravity_affects.strength = FRUITS_GRAVITY_HOLD;
        } else {
            gravity_affects.strength = FRUITS_GRAVITY_UP;
        }

        sprite.color = if cut_affects.can_be_cut {
            Color::WHITE
        } else {
            Color::rgba(0.7, 0.7, 0.7, 0.5)
        };
    }
}
