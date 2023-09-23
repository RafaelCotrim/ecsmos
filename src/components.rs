use std::cmp::Ordering;

use bevy::prelude::*;

#[derive(Component)]
pub struct Vehicle{
    pub max_speed: f32,
    pub max_deceleration: f32,
    pub max_aceleration: f32,
    pub reaction_time: f32,
}

impl Default for Vehicle{
    fn default() -> Self {
        Self { max_speed: 10.0, max_deceleration: 0.5, max_aceleration: 0.5, reaction_time: 10.0 }
    }
}
#[derive(Component)]
pub struct Leader(pub Entity);