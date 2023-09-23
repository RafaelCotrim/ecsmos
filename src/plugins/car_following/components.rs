use bevy::prelude::*;

#[derive(Component)]
pub struct Leader(pub Entity);

#[derive(Component)]
pub struct KraussVehicle{
    pub max_speed: f32,
    pub max_deceleration: f32,
    pub max_aceleration: f32,
    pub reaction_time: f32,
}

impl Default for KraussVehicle{
    fn default() -> Self {
        Self { max_speed: 10.0, max_deceleration: 0.5, max_aceleration: 0.5, reaction_time: 10.0 }
    }
}