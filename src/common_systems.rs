use bevy::prelude::*;
use crate::common_components::{GravityAffects, Velocity};
use crate::controls::Movement;
use crate::{PLAYER_GRAVITY, PLAYER_JUMP};
use crate::player_plugin::{IsOnWall, Player};

pub struct CommonSystems;

impl Plugin for CommonSystems {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_with_velocity_system)
            .add_system(gravity_system);
    }
}

fn move_with_velocity_system(
    mut query: Query<(&mut Transform, &Velocity)>
) {
    for (mut tf, vl) in query.iter_mut() {
        let mut translation: &mut Vec3 = &mut tf.translation;
        translation.x += vl.x;
        translation.y += vl.y;
    }
}

fn gravity_system(
    mut query: Query<(&mut Velocity, &GravityAffects), Without<Player>>,
) {
    for (mut vl, ga) in query.iter_mut() {
        vl.y -= ga.strength;
    }
}