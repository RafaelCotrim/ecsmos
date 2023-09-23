use std::cmp::Ordering;

use bevy::prelude::*;

#[derive(Component)]
pub struct Route(pub usize);

#[derive(Component)]
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