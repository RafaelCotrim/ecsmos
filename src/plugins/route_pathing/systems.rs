use bevy::prelude::*;

use crate::components::Vehicle;

use super::{components::*, resources::RoutingTable};



pub fn route_movement_system(
    pathing: Res<RoutingTable>,
    mut query: Query<(&mut PathPosition, &PathVelocity, &Route), With<Vehicle>>,
) {
    for (mut pos, PathVelocity(v), Route(route_id)) in &mut query {
        
        let route = pathing.routes.get(*route_id).unwrap();

        loop {

            let path_idx = route.paths.get(pos.path).unwrap();
            let path = pathing.paths.get(*path_idx).unwrap();
            let mut dx = *v;

            // Trivial case: The vehicle movement is cofined to 1 segment

            let (start, end) = path.segment(pos.segment);

            let segment_legth = end.distance(start);

            if pos.distance + dx <= segment_legth {
                pos.distance += dx;
                break;
            }

            // The vehicle movement goes past to another segment in the same path

            while path.segment_len() > pos.segment + 1 {
                pos.segment += 1;
                let (start, end) = path.segment(pos.segment);

                dx -= segment_legth - pos.distance;
                pos.distance = 0.;

                let segment_legth = end.distance(start);

                if pos.distance + dx <= segment_legth {
                    pos.distance += dx;
                    dx -= dx;
                    break;
                }
            }

            if dx == 0.{
                break;
            }

            // The vehicle goes past the end of one path and into another

            if route.paths.len() > pos.path + 1{
                pos.path += 1;
                pos.segment = 0;
                pos.distance = 0.;
                let (start, end) = path.segment(pos.segment);
                dx -= end.distance(start);
            } else {
                pos.path = 0;
                pos.distance = 0.;
                pos.segment = 0;
                break;
            }
        }
    }
}

pub fn transform_update_system(
    pathing: Res<RoutingTable>,
    mut query: Query<(&PathPosition, &mut Transform), With<Vehicle>>,
) {
    for (pos, mut transform) in &mut query {
        let path = pathing.paths.get(pos.path).unwrap();
        let (start, end) = path.segment(pos.segment);
        let length = end.distance(start);

        let complition = pos.distance / length;

        let point = start.lerp(end, complition);

        transform.translation = Vec3::new(point.x, point.y, 0.)
    }
}

const COLORS: &'static[Color] = &[Color::RED, Color::GREEN, Color::BLUE];

pub fn draw_paths(pathing: Res<RoutingTable>, mut gizmos: Gizmos) {

    let mut i = 0;

    for path in &pathing.paths {
        gizmos.linestrip_2d(path.points.to_owned(), COLORS[i % COLORS.len()]);
        i += 1;
    }
}
