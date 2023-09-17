use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow, utils::HashMap, math::vec2};

use crate::{components::*, resources::RoutingTable};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Stage {
    Simulation,
    Rendering   
}






// Setup

pub fn add_resources(mut commands: Commands){
    commands.insert_resource(RoutingTable::default());
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle{
        ..default()
    });
}

pub fn add_vehicles(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    commands.spawn((
        Vehicle,  
        RoutePosition { distance:0., route: 0, path: 0, segment: 0 },
        RouteVelocity(5.),
        SpriteBundle{ 
            transform: Transform::from_xyz(0., 0., 0.0),
            texture: asset_server.load("sprites/green.png"),
            ..default()
        }
    ));
}

// Update

pub fn route_movement_system(pathing: Res<RoutingTable>, mut query: Query<(&mut RoutePosition, &RouteVelocity), With<Vehicle>>) {
    for (mut pos, RouteVelocity(dx)) in &mut query {
        let path = pathing.paths.get(pos.path).unwrap();
        let mut seg_id =  pos.segment;
        let (start, end) = path.segment(seg_id);

        let segment_legth = end.distance(start);

        if pos.distance + dx <= segment_legth{
            pos.distance += dx;
            return;
        }

        while path.segment_len() > seg_id + 1{
            seg_id += 1;
            let (start, end) = path.segment(seg_id);

            let dx = dx - (segment_legth - pos.distance);
            pos.distance = 0.;

            let segment_legth = end.distance(start);

            if pos.distance + dx <= segment_legth{
                pos.distance += dx;
                pos.segment = seg_id; 
                return;
            }
        }

        pos.distance = 0.;
        pos.segment = 0;
    }
}

pub fn transform_update_system(pathing: Res<RoutingTable>, mut query: Query<(&RoutePosition, &mut Transform), With<Vehicle>>){
    for (pos, mut transform) in &mut query {
        let path = pathing.paths.get(pos.path).unwrap();
        let (start, end) = path.segment(pos.segment);
        let length = end.distance(start);

        let complition = pos.distance/length;

        let point = start.lerp(end, complition);

        transform.translation = Vec3::new(point.x, point.y, 0.)
    }
}


pub fn draw_paths(pathing: Res<RoutingTable>, mut gizmos: Gizmos, time: Res<Time>){

    for ( path) in &pathing.paths {
        gizmos.linestrip_2d(path.points.to_owned(), Color::RED);
    }
}