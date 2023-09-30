pub mod components;
pub mod resources;
pub mod systems;
pub mod events;

use bevy::prelude::*;

use crate::systems::Stage;

use self::{
    resources::RoutingTable,
    systems::*, events::PathChangedEvent,
};

pub struct RoutePathingPlugin;

impl Plugin for RoutePathingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoutingTable>()
            .add_event::<PathChangedEvent>()
            .add_systems(Update, (apply_intent_system, route_movement_system).chain().in_set(Stage::Movement))
            .add_systems(
                Update,
                transform_update_system.in_set(Stage::Rendering),
            )
            .add_systems(Update, draw_paths.in_set(Stage::Rendering));
    }
}
