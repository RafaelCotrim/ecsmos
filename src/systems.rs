use bevy::prelude::*;
use itertools::Itertools;

use crate::{components::*, plugins::route_pathing::{components::{PathPosition, Route, PathVelocity}, resources::RoutingTable}};

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
        Vehicle::default(),
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
        Vehicle::default(),
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

pub fn compute_leaders(
    mut commands: Commands,
    query: Query<(Entity, &PathPosition), With<Vehicle>>,
) {
    let groups = query.iter().group_by(|(_, pos)| pos.path);

    for (_, g) in groups.into_iter() {
        let mut last: Option<Entity> = None;

        for (e, _) in g.sorted_by(|(_, a), (_, b)| Ord::cmp(a, b).reverse()) {
            if let Some(leader) = last {
                commands.entity(e).insert(Leader(leader));
            }

            last = Some(e);
        }
    }
}

// Update

pub fn car_following(
    pathing: Res<RoutingTable>,
    mut cars: Query<(&mut PathVelocity, &Vehicle, &PathPosition)>,
    followers: Query<(Entity, &Leader), With<Vehicle>>
) {
    for (follower, Leader(leader)) in &followers {
        
        let leader_velocity: f32;
        let leader_segment: usize;
        let leader_distance: f32;

        {
            let (leader_path_velocity,_ , leader_position) = cars.get(*leader).unwrap();

            leader_velocity = leader_path_velocity.0;
            leader_segment = leader_position.segment;
            leader_distance = leader_position.distance;
        }

        let (mut follower_path_velocity, follower_vehicle, follower_pos) = cars.get_mut(follower).unwrap();

        let path = &pathing.paths[follower_pos.path];

        let gap = path.length_between(follower_pos.segment, leader_segment)
            + leader_distance
            - follower_pos.distance;

        let velocity = follower_path_velocity.0;

        let denomenator = ((leader_velocity + velocity) / (2.0 * follower_vehicle.max_deceleration))
            + follower_vehicle.reaction_time;
        let safe_speed =
            leader_velocity + (gap - leader_velocity * follower_vehicle.reaction_time) / denomenator;


        follower_path_velocity.0 = vec![
            safe_speed,
            follower_vehicle.max_aceleration + velocity,
            follower_vehicle.max_speed,
        ]
        .iter()
        .fold(f32::INFINITY, |a, &b| a.min(b));
    }
}
