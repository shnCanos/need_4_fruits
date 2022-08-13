use bevy::prelude::*;
use bevy::utils::{HashMap};
use crate::{FRUIT_SPEED, FRUITS_GRAVITY, FRUITS_SCALE, FRUITS_SIZE, TexturesHandles};
use rand::{Rng, thread_rng};
use crate::common_components::{GravityAffects, IsOnWall, Velocity, Walls};


//region Plugin Boilerplate
pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_fruit_system)
            .add_system(fruit_corners_system)
            .add_system(fruits_reach_bottom_system)
            .insert_resource(LastWallState(HashMap::new()));
    }
}
//endregion

//region Fruit Only Components
#[derive(Component)]
pub struct Fruit;

#[derive(Component)]
pub struct CutAffects {
    pub is_cut: bool,
}
//endregion

//region Fruit Only Resources
// This is a measure against the fruits despawning
// The moment they spawn
pub struct LastWallState(pub HashMap<Entity, Option<Walls>>);
//endregion

fn spawn_fruit_system(
    mut commands: Commands,
    window: Res<Windows>,
    textures: Res<TexturesHandles>
) {
    let number_of_fruits = textures.fruits.len();
    let index_of_fruit = thread_rng().gen_range(0..number_of_fruits);
    let texture = textures.fruits[index_of_fruit].clone();

    let window = window.get_primary().unwrap();
    let y_spawn_position = -window.height() / 2. - 50.;
    let x_spawn_position = thread_rng().gen_range((-window.width() / 2.)..(window.width() / 2.));

    commands.spawn_bundle(
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(x_spawn_position, y_spawn_position, 0.0),
                scale: FRUITS_SCALE,
                ..Default::default()
            },
            ..Default::default()
        }
    )
        .insert(Velocity { x: 0., y: FRUIT_SPEED })
        .insert(GravityAffects { strength: FRUITS_GRAVITY, dashing: false, is_player: false })
        .insert(IsOnWall(Some(Walls::Floor))) // Fruits spawn below the window
        .insert(Fruit)
        .insert(CutAffects {is_cut: false});
}

fn fruit_corners_system(
    mut query: Query<(&mut Transform, &mut IsOnWall), With<Fruit>>,
    window: Res<Windows>,
) {
    for (mut tf, mut is_on_wall) in query.iter_mut() {
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

fn fruits_reach_bottom_system (
    mut commands: Commands,
    mut query: Query<(Entity, &IsOnWall), With<Fruit>>,
    mut last_state: ResMut<LastWallState>,
) {

    for (entity, wall) in query.iter() {
        // If the last wall state hasn't been registered yet
        if !last_state.0.contains_key(&entity) {
            last_state.0.insert(entity, wall.0);
            return;
        }

        // If the last wall state is equal to the current state
        // (I had to write it like this due to some weird compile
        // Time error I couldn't get rid of)
        let current_state = &wall.0;
        if current_state == last_state.0.get(&entity).unwrap() {
            return;
        }

        // If the fruit hits the floor
        if let Some(_) = wall.0 {
            // Remove it from LastWallState
            last_state.0.remove(&entity);

            // Despawn it
            commands.entity(entity).despawn();

        } else {
            last_state.0.insert(entity, None);
        }
    }
}