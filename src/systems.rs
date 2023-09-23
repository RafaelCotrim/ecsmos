use bevy::prelude::*;

use crate::{components::*, plugins::{route_pathing::components::{PathPosition, Route, PathVelocity}, car_following::components::{Leader, KraussVehicle}}};

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

    let leader = commands.spawn((
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
    )).id();

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
        Leader(leader)
    ));
}

// pub fn compute_leaders(
//     mut commands: Commands,
//     query: Query<(Entity, &PathPosition), With<Vehicle>>,
// ) {
//     let groups = query.iter().group_by(|(_, pos)| pos.path);

//     for (_, g) in groups.into_iter() {
//         let mut last: Option<Entity> = None;

//         for (e, _) in g.sorted_by(|(_, a), (_, b)| Ord::cmp(a, b).reverse()) {
//             if let Some(leader) = last {
//                 commands.entity(e).insert(Leader(leader));
//             }

//             last = Some(e);
//         }
//     }
// }

// Update
