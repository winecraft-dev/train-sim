use bevy::prelude::*;

use crate::track::{TrackSegment, error::TrackError};

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

    pub fn on_track(e_track: Entity, segments: Query<&TrackSegment>) -> Result<Self, TrackError> {
        let track = match segments.get(e_track) {
            Ok(s) => s,
            Err(_) => return Err(TrackError::BrokenSegmentReference(e_track)),
        };
        let length = track.length();

        Ok(Self {
            track: e_track,
            distance: length / 2.0,
            direction: Direction::FacingB,
        })
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
