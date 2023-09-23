use bevy::prelude::*;

use crate::components::Vehicle;

use super::{resources::RoutingTable, components::*};

pub fn route_movement_system(
    pathing: Res<RoutingTable>,
    mut query: Query<(&mut PathPosition, &PathVelocity), With<Vehicle>>,
) {
    for (mut pos, PathVelocity(dx)) in &mut query {
        let path = pathing.paths.get(pos.path).unwrap();
        let mut seg_id = pos.segment;
        let (start, end) = path.segment(seg_id);

        let segment_legth = end.distance(start);

        if pos.distance + dx <= segment_legth {
            pos.distance += dx;
            continue;
        }

        while path.segment_len() > seg_id + 1 {
            seg_id += 1;
            let (start, end) = path.segment(seg_id);

            let dx = dx - (segment_legth - pos.distance);
            pos.distance = 0.;

            let segment_legth = end.distance(start);

            if pos.distance + dx <= segment_legth {
                pos.distance += dx;
                pos.segment = seg_id;
                return;
            }
        }

        pos.distance = 0.;
        pos.segment = 0;
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

pub fn draw_paths(pathing: Res<RoutingTable>, mut gizmos: Gizmos) {
    for path in &pathing.paths {
        gizmos.linestrip_2d(path.points.to_owned(), Color::RED);
    }
}