use bevy::prelude::*;

use crate::{
    components::*,
    plugins::{
        car_following::components::KraussVehicle,
        route_pathing::components::{PathPosition, PathVelocity, Route},
    },
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Stage {
    Compute,
    Movement,
    Rendering,
}

// Setup

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

pub fn add_vehicles(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Vehicle,
        KraussVehicle::default(),
        PathPosition {
            distance: 500.,
            path: 0,
            segment: 0,
        },
        Route(1),
        PathVelocity(3.),
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.0),
            texture: asset_server.load("sprites/red.png"),
            ..default()
        },
    ));

    commands.spawn((
        Vehicle,
        KraussVehicle::default(),
        PathPosition {
            distance: 0.,
            path: 0,
            segment: 0,
        },
        Route(1),
        PathVelocity(8.),
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.0),
            texture: asset_server.load("sprites/green.png"),
            ..default()
        },
        // Leader(leader)
    ));
}
