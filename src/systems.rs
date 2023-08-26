use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Stage {
    Simulation,
    Rendering   
}

pub fn add_vehicles(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        ..default()
    });

    commands.spawn((
        Vehicle,  
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 5.0, y: 2.0 },
        SpriteBundle{
            transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            texture: asset_server.load("sprites/green.png"),
            ..default()
        }
    ));
}



pub fn movement_system(mut query: Query<(&mut Transform, &Velocity), With<Vehicle>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += Vec3::new(velocity.x, velocity.y, 0.0);
    }
}

pub fn confine_system(mut query: Query<&mut Transform, With<Vehicle>>, window_query: Query<&Window, With<PrimaryWindow>>){

    let window = window_query.get_single().unwrap();

    for mut transform in &mut query {
        if transform.translation.x < 0.0 {
            transform.translation.x = window.width();
        }

        if transform.translation.x > window.width() {
            transform.translation.x = 0.0;
        }

        if transform.translation.y < 0.0 {
            transform.translation.y = window.height();
        }

        if transform.translation.y > window.height() {
            transform.translation.y = 0.0;
        }
    }
}
