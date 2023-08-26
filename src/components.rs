use bevy::prelude::*;

#[derive(Component)]
pub struct Vehicle;

#[derive(Component)]
pub struct Position { pub x: f32, pub y: f32 }

#[derive(Component, Clone, Copy)]
pub struct Velocity { pub x: f32, pub y: f32 }