use bevy::prelude::*;
use ecsmos::{systems::*, plugins::route_pathing::RoutePathingPlugin};

fn main() {
    App::new()

    .add_plugins(DefaultPlugins)
    .add_plugins(RoutePathingPlugin)
    
    .configure_set(Update, Stage::Compute)
    .configure_set(Update, Stage::Movement.after(Stage::Compute))
    .configure_set(Update, Stage::Rendering.after(Stage::Movement))


    .add_systems(Startup, (setup_camera, add_vehicles).chain())

    // .add_systems(Update, car_following.after(transform_update_system))
    .run();
}