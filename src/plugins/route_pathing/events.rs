use bevy::prelude::*;

#[derive(Event)]
pub struct PathChangedEvent{
    pub entity: Entity,
    pub preivous: usize,
    pub current: usize
}