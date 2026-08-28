use bevy::prelude::*;

pub mod cursor;
pub mod error;
pub mod projector;

use crate::loc::projector::Projector;

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_transforms).chain());
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Location {
    pub track: Entity,
    pub distance: f32,
    pub direction: Direction,
}

impl Location {
    pub fn new(e_track: Entity) -> Self {
        Self {
            track: e_track,
            distance: 0.0,
            direction: Direction::FacingB,
        }
    }

    pub fn with_distance(mut self, distance: f32) -> Self {
        self.distance = distance;
        self
    }
    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}

fn add_transforms(
    mut commands: Commands,
    locations: Query<(Entity, &Location), Without<Transform>>,
    projector: Projector,
) {
    // if let TrackStatus::Loading = *track_status {
    //     return;
    // }

    for (e, loc) in locations {
        let pos = match projector.project(*loc) {
            Ok(v3) => v3,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        commands.entity(e).insert(Transform::from_translation(pos));
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    FacingA,
    FacingB,
}

impl Direction {
    pub fn flip(&mut self) {
        match self {
            Direction::FacingA => *self = Direction::FacingB,
            Direction::FacingB => *self = Direction::FacingA,
        }
    }
}
