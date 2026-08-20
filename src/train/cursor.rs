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

impl TrackTraversal {
    fn flip(&mut self) {
        match self {
            TrackTraversal::FacingA => *self = TrackTraversal::FacingB,
            TrackTraversal::FacingB => *self = TrackTraversal::FacingA,
        }
    }
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
        self.traverse(segments, switches);
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
        self.distance -= offset;
        self.traverse(segments, switches);
        self.clone()
    }

    fn traverse(&mut self, segments: Query<&TrackSegment>, switches: Query<&TrackSwitch>) {
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

            self.traverse_next(e_next, e_switch, segments, overflow_distance);
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
        e_switch: Entity, // node
        segments: Query<&TrackSegment>,
        overflow_distance: f32,
    ) {
        let last_track = segments.get(self.track).unwrap();
        let next_track = segments.get(e_next).unwrap();

        self.track = e_next;
        self.select_direction(last_track, next_track);
        if next_track.nodes.0 == e_switch {
            self.distance = overflow_distance.abs();
        } else {
            self.distance = next_track.length() - overflow_distance.abs();
        }
    }

    fn select_direction(&mut self, last_track: &TrackSegment, next_track: &TrackSegment) {
        if last_track.nodes.0 == next_track.nodes.0 {
            self.traversal.flip();
        } else if last_track.nodes.1 == next_track.nodes.1 {
            self.traversal.flip();
        }
    }
}
