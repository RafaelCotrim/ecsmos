use bevy::prelude::*;

#[derive(Component)]
pub struct Vehicle;

#[derive(Component)]
pub struct RoutePosition { 
    pub distance: f32, 
    pub route: usize,
    pub path: usize,
    pub segment: usize,
}

#[derive(Component)]
pub struct RouteVelocity(pub f32);

#[derive(Component, Clone, Copy)]
pub struct Velocity { pub x: f32, pub y: f32 }