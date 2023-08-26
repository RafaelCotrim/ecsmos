use bevy::prelude::*;
use ecsmos::systems::*;

fn main() {
    App::new()

    .add_plugins(DefaultPlugins)
    .configure_set(Update, Stage::Rendering)
    .configure_set(Update, Stage::Simulation.after(Stage::Rendering))

    .add_systems(Startup, add_vehicles)

    .add_systems(Update, movement_system.in_set(Stage::Simulation))
    .add_systems(Update, confine_system.in_set(Stage::Simulation).after(movement_system))
    // .add_systems(drawing_system)
    .run();
}
