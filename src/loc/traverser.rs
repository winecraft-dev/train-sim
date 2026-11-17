use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, BrokenNodeReference, BrokenSegmentReference, NoNeighborSegment},
    loc::{Dir, Loc},
    track::{TrackSegment, switch::NodeVariant},
};

#[derive(SystemParam)]
pub struct Traverser<'w, 's> {
    segments: Query<'w, 's, &'static TrackSegment>,
    nodes: Query<'w, 's, &'static NodeVariant>,
}

impl<'w, 's> Traverser<'w, 's> {
    pub fn traverse(&self, mut loc: Loc, distance: f32) -> Result<Loc, SimError> {
        loc.distance += distance;

        loop {
            let current_track = match self.segments.get(loc.track) {
                Ok(s) => s,
                Err(_) => return Err(BrokenSegmentReference(loc.track)),
            };
            let (e_switch, overflow_distance) = match self.exited(&loc, current_track) {
                Some(exit) => exit,
                None => return Ok(loc),
            };

            let switch = match self.nodes.get(e_switch) {
                Ok(s) => s,
                Err(_) => return Err(BrokenNodeReference(e_switch)),
            };
            let e_next = match switch.next_segment(loc.track) {
                Some(segment) => segment,
                None => return Err(NoNeighborSegment),
            };

            let next_track = match self.segments.get(e_next) {
                Ok(s) => s,
                Err(_) => return Err(BrokenSegmentReference(e_next)),
            };

            loc.track = e_next;
            loc.dir = select_direction(loc.dir, current_track, next_track);
            loc.distance = if next_track.nodes.0 == e_switch {
                overflow_distance.abs()
            } else {
                next_track.length - overflow_distance.abs()
            };
        }
    }

    pub fn next_track(&self, loc: &mut Loc) -> Result<(), SimError> {
        let current_track = match self.segments.get(loc.track) {
            Ok(s) => s,
            Err(_) => return Err(BrokenSegmentReference(loc.track)),
        };
        let e_switch = match loc.dir {
            Dir::A => current_track.nodes.0,
            Dir::B => current_track.nodes.1,
        };
        let switch = match self.nodes.get(e_switch) {
            Ok(n) => n,
            Err(_) => return Err(BrokenNodeReference(e_switch)),
        };
        let e_next = match switch.next_segment(loc.track) {
            Some(s) => s,
            None => return Err(NoNeighborSegment),
        };
        let next_track = match self.segments.get(e_next) {
            Ok(s) => s,
            Err(_) => return Err(BrokenSegmentReference(e_next)),
        };
        loc.track = e_next;
        loc.dir = select_direction(loc.dir, current_track, next_track);
        loc.distance = if next_track.nodes.0 == e_switch {
            0.0
        } else {
            next_track.length
        };
        Ok(())
    }

    fn exited(&self, loc: &Loc, track: &TrackSegment) -> Option<(Entity, f32)> {
        let length = track.length;
        if loc.distance < 0.0 {
            let node_a = track.nodes.0;
            return Some((node_a, loc.distance));
        } else if loc.distance > length {
            let node_b = track.nodes.1;
            return Some((node_b, loc.distance - length));
        }
        None
    }
}

pub fn select_direction(dir: Dir, current_track: &TrackSegment, next_track: &TrackSegment) -> Dir {
    if current_track.nodes.0 == next_track.nodes.0 {
        dir.flip()
    } else if current_track.nodes.1 == next_track.nodes.1 {
        dir.flip()
    } else {
        dir
    }
}
