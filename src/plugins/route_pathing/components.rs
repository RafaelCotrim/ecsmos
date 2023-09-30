use std::cmp::Ordering;

use bevy::prelude::*;

#[derive(Component)]
pub struct Route(pub usize);

#[derive(Component, Clone, Copy)]
pub struct PathPosition { 
    pub distance: f32, 
    pub path: usize,
    pub segment: usize,
}

impl Eq for PathPosition {
    
}

impl Ord for PathPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.segment != other.segment {
            return self.segment.cmp(&other.segment);
        }
        self.distance.total_cmp(&other.distance)
    }
}

impl PartialOrd for PathPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathPosition {
    fn eq(&self, other: &Self) -> bool {
        self.segment == other.segment && self.distance == self.distance
    }
}

#[derive(Component)]
pub struct PathVelocity(pub f32);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct PathPositionIntent{
    pub distance: f32, 
    pub path: usize,
    pub segment: usize,
}

impl PathPositionIntent {
    pub fn apply_to(&self, pos: &mut PathPosition){
        pos.path = self.path;
        pos.segment = self.segment;
        pos.distance = self.distance;
    }
    
}

impl From<&PathPosition> for PathPositionIntent {
    fn from(value: &PathPosition) -> Self {
        Self { distance:value.distance, path: value.path, segment: value.segment }
    }
}