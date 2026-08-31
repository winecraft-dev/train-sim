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

pub type FacingLocation = (Location, Direction);

#[derive(Component, Debug, Clone, Copy)]
pub struct Location {
    pub track: Entity,
    pub distance: f32,
}

impl Location {
    pub fn new(e_track: Entity) -> Self {
        Self {
            track: e_track,
            distance: 0.0,
        }
    }

    pub fn with_distance(mut self, distance: f32) -> Self {
        self.distance = distance;
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

#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    FacingA,
    #[default]
    FacingB,
}

impl Direction {
    pub fn flip(self) -> Self {
        match self {
            Direction::FacingA => Direction::FacingB,
            Direction::FacingB => Direction::FacingA,
        }
    }
}
