use crate::game::common_components::{GravityAffects, IsOnWall, TimeAnimation, Velocity, Walls};
use crate::game::common_systems::{RestartEvent};
use crate::game::{
    Score, TexturesHandles, DEFAULT_FRUIT_SPAWN_TIME, FRUITS_GRAVITY, FRUITS_SCALE, FRUITS_SIZE,
    FRUIT_HORIZONTAL_MARGIN, FRUIT_SPEED, MAX_FRUIT_PIECE_SPEED, NUMBER_OF_FRUIT_PIECES,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

//region Plugin Boilerplate
pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_fruit_system
                .after(fruit_corners_system)
                .after(fruits_reach_bottom_system),
        )
        .add_system(fruit_corners_system)
        .add_system(fruits_reach_bottom_system)
        .add_system(fruits_get_cut_system)
        .add_system(fruit_part_eliminate_system)
        .insert_resource(FruitSpawnerTimer(Timer::from_seconds(
            DEFAULT_FRUIT_SPAWN_TIME,
            false,
        )));
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
}
//endregion

pub struct FruitSpawnerTimer(pub Timer);
//endregion

fn spawn_fruit_system(
    mut commands: Commands,
    window: Res<Windows>,
    textures: Res<TexturesHandles>,
    mut fruitspawner: ResMut<FruitSpawnerTimer>,
    time: Res<Time>,
    score: ResMut<Score>, // The higher the score, the faster fruits spawn
) {
    // If the timer isn't finished, don't do anything
    if !fruitspawner.0.finished() {
        fruitspawner.0.tick(time.delta());
        return;
    }

    // dbg!(score.0);

    // Respawn timer taking the score into account
    // let timer_time = DEFAULT_FRUIT_SPAWN_TIME / (score.0 as f32);
    let timer_time = (score.0 as f32 * 0.2 + 5.) / (score.0 as f32 + 5.);
    fruitspawner.0 = Timer::from_seconds(timer_time, false);

    // Random fruit generation
    let number_of_fruits = textures.fruits.len();
    let index_of_fruit = thread_rng().gen_range(0..number_of_fruits);
    let texture = textures.fruits[index_of_fruit].clone();

    // Random position generation
    let window = window.get_primary().unwrap();
    let y_spawn_position = -window.height() / 2. - 50.;
    let x_spawn_position = thread_rng().gen_range(
        (-window.width() / 2. + FRUIT_HORIZONTAL_MARGIN)
            ..(window.width() / 2. - FRUIT_HORIZONTAL_MARGIN),
    );

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
            strength: FRUITS_GRAVITY,
        })
        .insert(IsOnWall(None))
        .insert(Fruit {
            texture_id: index_of_fruit,
        })
        .insert(CutAffects { is_cut: false })
        .insert(TimeAnimation::from_callback(|tf, _, t| {
            tf.rotation = Quat::from_rotation_z(t * 4.0);
        }));
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
    mut restart_events : EventWriter<RestartEvent>
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

        //TODO! Fruits dying animation
        // In progress....

        for part_id in 0..(NUMBER_OF_FRUIT_PIECES as usize) {
            let fruit_atlas = textures.fruits_pieces_texture_atlas[fruit.texture_id].clone();
            //dbg!(&fruit_atlas);

            // TODO! Math to put each piece in its place
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
                    strength: FRUITS_GRAVITY * 2.,
                }) // The pieces of the fruit are affected by gravity
                .insert(IsOnWall(None))
                .insert(TimeAnimation {
                    callback: |tf, data, t| {
                        tf.rotation = Quat::from_rotation_z(t * data[0]);
                        tf.scale = FRUITS_SCALE * 0.75 * (1. - 0.5 * t);
                    },
                    data: vec![thread_rng().gen_range(2.0..4.0) * if thread_rng().gen_bool(0.5) {1.} else {-1.} ],
                    time: 0.,
                }); // We check whether it hit the floor to despawn
        }

        commands.entity(entity).despawn();
    }
}
