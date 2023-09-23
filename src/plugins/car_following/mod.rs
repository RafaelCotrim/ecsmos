pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::systems::Stage;

use self::systems::{car_following, compute_leaders_on_add};

pub struct CarFollowingPlugin;

impl Plugin for CarFollowingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, car_following.in_set(Stage::Compute))
        .add_systems(Update, compute_leaders_on_add.before(Stage::Compute));
    }
}