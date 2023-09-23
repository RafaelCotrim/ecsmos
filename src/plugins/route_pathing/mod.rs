pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use crate::systems::Stage;

use self::{
    resources::RoutingTable,
    systems::{draw_paths, route_movement_system, transform_update_system},
};

pub struct RoutePathingPlugin;

impl Plugin for RoutePathingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoutingTable>()

            .add_systems(Update, route_movement_system.in_set(Stage::Movement))
            .add_systems(
                Update,
                transform_update_system.in_set(Stage::Rendering),
            )
            .add_systems(Update, draw_paths.in_set(Stage::Rendering));
    }
}
