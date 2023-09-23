use bevy::prelude::*;
use itertools::Itertools;

use crate::{plugins::route_pathing::{resources::RoutingTable, components::*}, components::Vehicle};

use super::components::{Leader, KraussVehicle};

pub fn car_following(
    pathing: Res<RoutingTable>,
    mut cars: Query<(&mut PathVelocity, &KraussVehicle, &PathPosition), With<Vehicle>>,
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


pub fn compute_leaders_on_add(
    mut commands: Commands,
    query: Query<(Entity, &PathPosition), (With<Vehicle>, Added<KraussVehicle>)>,
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

pub fn compute_leaders_on_change(){
    
}