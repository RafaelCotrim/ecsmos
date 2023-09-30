use bevy::prelude::*;

use crate::components::Vehicle;

use super::{components::*, resources::RoutingTable, events::PathChangedEvent};



pub fn route_movement_system(
    pathing: Res<RoutingTable>,
    mut query: Query<(Entity,&mut PathPosition, &PathVelocity, &Route), With<Vehicle>>,
    // mut commands: Commands,
    mut ev_path_changed: EventWriter<PathChangedEvent>
) {
    for (entity, mut pos, PathVelocity(speed), Route(route_id)) in &mut query {
        
        let route = pathing.routes.get(*route_id).unwrap();
        // let mut pos = PathPositionIntent::from(pos.as_ref());
        let  old_pos = pos.clone();

        loop {

            let path_idx = route.paths.get(pos.path).unwrap();
            let path = pathing.paths.get(*path_idx).unwrap();
            let mut dx = *speed;

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
    
        if old_pos.path != pos.path {
            ev_path_changed.send(PathChangedEvent { entity, preivous: old_pos.path, current: pos.path })
        }

        // commands.entity(entity).insert(pos);
    }
}

pub fn apply_intent_system(mut query: Query<(Entity, &mut PathPosition, &PathPositionIntent), With<Vehicle>>, mut commands: Commands){
    
    for (entity, mut pos, intent) in &mut query{
        intent.apply_to(&mut pos);

        commands.entity(entity).remove::<PathPositionIntent>();
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
