use bevy::prelude::*;
use crate::common_components::{GravityAffects, Velocity};
use crate::player_plugin::{Player};

pub struct CommonSystems;

impl Plugin for CommonSystems {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_with_velocity_system)
            .add_system(gravity_system);
    }
}

fn move_with_velocity_system(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut tf, vl) in query.iter_mut() {
        let mut translation: &mut Vec3 = &mut tf.translation;

        // Multiply by 60 and time delta in order to
        // Make the game independent of frames
        // (My monitor is 60hz so that's the default)
        translation.x += vl.x * 60. * time.delta_seconds();
        translation.y += vl.y * 60. * time.delta_seconds();
    }
}

fn gravity_system(
    mut query: Query<(&mut Velocity, &GravityAffects), Without<Player>>,
) {
    for (mut vl, ga) in query.iter_mut() {
        vl.y -= ga.strength;
    }
}