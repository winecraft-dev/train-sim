use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    switch::TrackSwitch,
    track::{
        TrackSegment,
        error::TrackError,
        loc::{Direction, Location},
    },
};

#[derive(SystemParam)]
pub struct TrackCursor<'w, 's> {
    segments: Query<'w, 's, &'static TrackSegment>,
    switches: Query<'w, 's, &'static TrackSwitch>,
}

impl<'w, 's> TrackCursor<'w, 's> {
    pub fn traverse(&self, mut loc: Location, distance: f32) -> Result<Location, TrackError> {
        loc.distance += distance;

        loop {
            let current_track = match self.segments.get(loc.track) {
                Ok(s) => s,
                Err(_) => return Err(TrackError::BrokenSegmentReference(loc.track)),
            };
            let (e_switch, overflow_distance) = match self.exited(&loc, current_track) {
                Some(exit) => exit,
                None => return Ok(loc),
            };

            let switch = match self.switches.get(e_switch) {
                Ok(s) => s,
                Err(_) => return Err(TrackError::BrokenNodeReference(e_switch)),
            };
            let e_next = match switch.next_segment(loc.track) {
                Some(segment) => segment,
                None => return Err(TrackError::NoNeighborSegment),
            };

            let next_track = match self.segments.get(e_next) {
                Ok(s) => s,
                Err(_) => return Err(TrackError::BrokenSegmentReference(e_next)),
            };

            loc.track = e_next;
            loc.direction = select_direction(loc.direction, current_track, next_track);
            loc.distance = if next_track.nodes.0 == e_switch {
                overflow_distance.abs()
            } else {
                next_track.length() - overflow_distance.abs()
            };
        }
    }

    fn exited(&self, loc: &Location, track: &TrackSegment) -> Option<(Entity, f32)> {
        let length = track.length();
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

pub fn select_direction(
    mut dir: Direction,
    current_track: &TrackSegment,
    next_track: &TrackSegment,
) -> Direction {
    if current_track.nodes.0 == next_track.nodes.0 {
        dir.flip();
    } else if current_track.nodes.1 == next_track.nodes.1 {
        dir.flip();
    }
    dir
}
