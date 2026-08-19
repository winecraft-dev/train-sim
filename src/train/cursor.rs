use bevy::prelude::*;

use crate::{
    switch::TrackSwitch,
    track::TrackSegment,
    train::axle::{Axle, AxleOffset},
};

#[derive(Debug, Clone, Copy)]
pub enum TrackTraversal {
    FacingA,
    FacingB,
}

pub type TrackCursor = Axle;

impl TrackCursor {
    pub fn apply_speed(
        &mut self,
        speed: f32,
        segments: Query<&TrackSegment>,
        switches: Query<&TrackSwitch>,
    ) {
        let speed = match self.traversal {
            TrackTraversal::FacingA => -speed,
            TrackTraversal::FacingB => speed,
        };
        self.distance += speed;
        self.traverse(segments, switches, speed);
    }

    pub fn next_offset(
        &mut self,
        offset: &AxleOffset,
        segments: Query<&TrackSegment>,
        switches: Query<&TrackSwitch>,
    ) -> Self {
        let offset = match self.traversal {
            TrackTraversal::FacingA => -offset.0,
            TrackTraversal::FacingB => offset.0,
        };
        self.distance += offset;
        self.traverse(segments, switches, offset);
        self.clone()
    }

    fn traverse(
        &mut self,
        segments: Query<&TrackSegment>,
        switches: Query<&TrackSwitch>,
        delta: f32,
    ) {
        loop {
            let current = segments.get(self.track).unwrap();
            let (e_switch, overflow_distance) = match self.exited(current) {
                Some(exit) => exit,
                None => break,
            };

            let switch = switches.get(e_switch).unwrap();
            let e_next = match switch.next_segment(self.track) {
                Some(segment) => segment,
                None => break,
            };

            self.traverse_next(e_next, e_switch, segments, delta, overflow_distance);
        }
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
        e_origin: Entity, // node
        segments: Query<&TrackSegment>,
        delta: f32,
        overflow_distance: f32,
    ) {
        self.track = e_next;

        let next_track = segments.get(e_next).unwrap();
        if next_track.nodes.0 == e_origin {
            self.traversal = if delta >= 0.0 {
                TrackTraversal::FacingB
            } else {
                TrackTraversal::FacingA
            };
            self.distance = overflow_distance;
        } else {
            self.traversal = if delta >= 0.0 {
                TrackTraversal::FacingA
            } else {
                TrackTraversal::FacingB
            };
            self.distance = next_track.length() + overflow_distance;
        }
    }
}
