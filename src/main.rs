use bevy::prelude::*;
use ecsmos::{systems::*, plugins::{route_pathing::RoutePathingPlugin, car_following::CarFollowingPlugin}};

fn main() {
    App::new()

    .add_plugins(DefaultPlugins)
    .add_plugins(RoutePathingPlugin)
    .add_plugins(CarFollowingPlugin)
    
    .configure_set(Update, Stage::Compute)
    .configure_set(Update, Stage::Movement.after(Stage::Compute))
    .configure_set(Update, Stage::Rendering.after(Stage::Movement))

    .add_systems(Startup, (setup_camera, add_vehicles))
    // .add_systems(Update, compute_leaders)
    .run();
}