use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow, utils::HashMap, math::vec2};

use crate::components::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Stage {
    Simulation,
    Rendering   
}

pub struct Path{
    pub id: String,
    pub points: Vec<Vec2>
}

impl Path {
    
    fn segment_len(&self) -> usize{
        self.points.len() - 1
    }

    fn segment(&self, n: usize) -> (Vec2, Vec2) {
        (self.points[n], self.points[n + 1])
    }
}

pub struct Route{
    pub id: String,
    pub paths: Vec<usize>
}
#[derive(Resource)]
pub struct RoutingTable{
    pub routes: Vec<Route>,
    pub paths: Vec<Path>,
}

impl Default for RoutingTable {
    fn default() -> Self {
        let mut paths = Vec::new();
        let mut routes = Vec::new();
        
        paths.push( Path { id: "p_1".to_owned(), points: vec![vec2(0., 300.), vec2(0., -50.), vec2(50., -150.), vec2(100., -200.0), vec2(200., -250.), vec2(300., -250.), vec2(400., -200.), vec2(450., -150.), vec2(500., -50.), vec2(500., 300.)] });

        routes.push( Route{ id:"r_1".to_owned(), paths: vec![0]});

        RoutingTable { paths, routes }
    }
}


// Setup

pub fn add_resources(mut commands: Commands){
    commands.insert_resource(RoutingTable::default());
}

pub fn add_vehicles(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle{
        // transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        ..default()
    });

    commands.spawn((
        Vehicle,  
        RoutePosition { distance:0., route: 0, path: 0, segment: 0 },
        RouteVelocity(5.),
        // Velocity { x: 5.0, y: 2.0 },
        SpriteBundle{ 
            transform: Transform::from_xyz(0., 0., 0.0),
            texture: asset_server.load("sprites/green.png"),
            ..default()
        }
    ));
}

// Update

pub fn movement_system(mut query: Query<(&mut Transform, &Velocity), With<Vehicle>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += Vec3::new(velocity.x, velocity.y, 0.0);
    }
}

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

pub fn confine_system(mut query: Query<&mut Transform, With<Vehicle>>, window_query: Query<&Window, With<PrimaryWindow>>){

    let window = window_query.get_single().unwrap();

    for mut transform in &mut query {
        if transform.translation.x < -window.width()/2.0{
            transform.translation.x = window.width()/2.0;
        }

        if transform.translation.x > window.width()/2.0 {
            transform.translation.x = -window.width()/2.0;
        }

        if transform.translation.y < -window.height()/2.0 {
            transform.translation.y = window.height()/2.0;
        }

        if transform.translation.y > window.height()/2.0 {
            transform.translation.y = -window.height()/2.0;
        }
    }
}

pub fn draw_paths(pathing: Res<RoutingTable>, mut gizmos: Gizmos, time: Res<Time>){

    for ( path) in &pathing.paths {
        gizmos.linestrip_2d(path.points.to_owned(), Color::RED);
    }

    // gizmos.cuboid(
    //     Transform::default().with_scale(Vec3::splat(10.)),
    //     Color::BLACK,
    // );

    // let sin = time.elapsed_seconds().sin() * 50.;
    // gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
    // gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);

    // // Triangle
    // gizmos.linestrip_gradient_2d([
    //     (Vec2::Y * 300., Color::BLUE),
    //     (Vec2::new(-255., -155.), Color::RED),
    //     (Vec2::new(255., -155.), Color::GREEN),
    //     (Vec2::Y * 300., Color::BLUE),
    // ]);

    // gizmos.rect_2d(
    //     Vec2::ZERO,
    //     time.elapsed_seconds() / 3.,
    //     Vec2::splat(300.),
    //     Color::BLACK,
    // );

    // // The circles have 32 line-segments by default.
    // gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
    // // You may want to increase this for larger circles.
    // gizmos.circle_2d(Vec2::ZERO, 300., Color::NAVY).segments(64);

    // // Arcs default amount of segments is linerarly interpolated between
    // // 1 and 32, using the arc length as scalar.
    // gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);
}