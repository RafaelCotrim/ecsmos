use bevy::prelude::*;
use ecsmos::systems::*;

fn main() {
    App::new()

    .add_plugins(DefaultPlugins)
    .configure_set(Update, Stage::Rendering)
    .configure_set(Update, Stage::Simulation.after(Stage::Rendering))


    .add_systems(Startup, (setup_camera, add_resources, add_vehicles))

    .add_systems(Update, route_movement_system.in_set(Stage::Simulation))
    .add_systems(Update, transform_update_system.in_set(Stage::Simulation).after(route_movement_system))

    .add_systems(Update, draw_paths)
    .run();
}