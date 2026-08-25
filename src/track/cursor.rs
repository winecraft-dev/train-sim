use bevy::prelude::*;

use crate::{
    switch::TrackSwitch,
    track::{
        TrackSegment,
        error::TrackError,
        loc::{Direction, Location},
    },
};

pub type TrackCursor = Location;

impl TrackCursor {
    pub fn apply_speed(
        &mut self,
        speed: f32,
        segments: Query<&TrackSegment>,
        switches: Query<&TrackSwitch>,
    ) -> Result<(), TrackError> {
        let speed = match self.direction {
            Direction::FacingA => -speed,
            Direction::FacingB => speed,
        };
        self.distance += speed;
        self.traverse(segments, switches)?;
        Ok(())
    }

    pub fn next_offset(
        &mut self,
        offset: f32,
        segments: Query<&TrackSegment>,
        switches: Query<&TrackSwitch>,
    ) -> Result<Self, TrackError> {
        let offset = match self.direction {
            Direction::FacingA => -offset,
            Direction::FacingB => offset,
        };
        self.distance -= offset;
        self.traverse(segments, switches)?;
        Ok(self.clone())
    }

    fn traverse(
        &mut self,
        segments: Query<&TrackSegment>,
        switches: Query<&TrackSwitch>,
    ) -> Result<(), TrackError> {
        loop {
            let current = match segments.get(self.track) {
                Ok(s) => s,
                Err(_) => return Err(TrackError::BrokenSegmentReference(self.track)),
            };
            let (e_switch, overflow_distance) = match self.exited(current) {
                Some(exit) => exit,
                None => break,
            };

            let switch = match switches.get(e_switch) {
                Ok(s) => s,
                Err(_) => return Err(TrackError::BrokenNodeReference(e_switch)),
            };
            let e_next = match switch.next_segment(self.track) {
                Some(segment) => segment,
                None => return Err(TrackError::NoNeighborSegment),
            };

            self.traverse_next(e_next, e_switch, segments, overflow_distance)?;
        }
        Ok(())
    }

    fn exited(&self, track: &TrackSegment) -> Option<(Entity, f32)> {
        let length = track.length();
        if self.distance < 0.0 {
            let node_a = track.nodes.0;
            return Some((node_a, self.distance));
        } else if self.distance > length {
            let node_b = track.nodes.1;
            return Some((node_b, self.distance - length));
        }
        None
    }

    fn traverse_next(
        &mut self,
        e_next: Entity,   // segment
        e_switch: Entity, // node
        segments: Query<&TrackSegment>,
        overflow_distance: f32,
    ) -> Result<(), TrackError> {
        let last_track = match segments.get(self.track) {
            Ok(s) => s,
            Err(_) => return Err(TrackError::BrokenSegmentReference(self.track)),
        };
        let next_track = match segments.get(e_next) {
            Ok(s) => s,
            Err(_) => return Err(TrackError::BrokenSegmentReference(e_next)),
        };

        self.track = e_next;
        self.select_direction(last_track, next_track);
        if next_track.nodes.0 == e_switch {
            self.distance = overflow_distance.abs();
        } else {
            self.distance = next_track.length() - overflow_distance.abs();
        }
        Ok(())
    }

    pub fn select_direction(&mut self, last_track: &TrackSegment, next_track: &TrackSegment) {
        if last_track.nodes.0 == next_track.nodes.0 {
            self.direction.flip();
        } else if last_track.nodes.1 == next_track.nodes.1 {
            self.direction.flip();
        }
    }
}
