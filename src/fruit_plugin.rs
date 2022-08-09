use bevy::prelude::*;
use crate::{FRUIT_SPEED, FRUITS_GRAVITY, FRUITS_SCALE, TexturesHandles,};
use rand::{Rng, thread_rng};
use crate::common_components::{GravityAffects, Velocity};

//region Plugin Boilerplate
pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_fruit_system);
    }
}
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
        .insert(GravityAffects { strength: FRUITS_GRAVITY, dashing: false, is_player: false });
}