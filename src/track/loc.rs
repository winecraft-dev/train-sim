use bevy::prelude::*;

use crate::track::TrackSegment;

#[derive(Component, Debug, Clone, Copy)]
pub struct Location {
    pub track: Entity,
    pub distance: f32,
    pub direction: Direction,
}

impl Location {
    pub fn on_track(e_track: Entity, segments: Query<&TrackSegment>) -> Self {
        let track = segments.get(e_track).unwrap();
        let length = track.length();

        Self {
            track: e_track,
            distance: length / 2.0,
            direction: Direction::FacingB,
        }
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
