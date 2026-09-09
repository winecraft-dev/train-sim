use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    loc::{Direction, FacingLocation, Location, error::LocError},
    track::{TrackSegment, switch::TrackSwitch},
};

#[derive(SystemParam)]
pub struct TrackCursor<'w, 's> {
    segments: Query<'w, 's, &'static TrackSegment>,
    switches: Query<'w, 's, &'static TrackSwitch>,
}

impl<'w, 's> TrackCursor<'w, 's> {
    pub fn traverse(
        &self,
        f_loc: FacingLocation,
        distance: f32,
    ) -> Result<FacingLocation, LocError> {
        let (mut loc, mut facing) = f_loc;
        loc.distance += distance;

        loop {
            let current_track = match self.segments.get(loc.track) {
                Ok(s) => s,
                Err(_) => return Err(LocError::BrokenSegmentReference(loc.track)),
            };
            let (e_switch, overflow_distance) = match self.exited(&loc, current_track) {
                Some(exit) => exit,
                None => return Ok((loc, facing)),
            };

            let switch = match self.switches.get(e_switch) {
                Ok(s) => s,
                Err(_) => return Err(LocError::BrokenNodeReference(e_switch)),
            };
            let e_next = match switch.next_segment(loc.track) {
                Some(segment) => segment,
                None => return Err(LocError::NoNeighborSegment),
            };

            let next_track = match self.segments.get(e_next) {
                Ok(s) => s,
                Err(_) => return Err(LocError::BrokenSegmentReference(e_next)),
            };

            loc.track = e_next;
            facing = select_direction(facing, current_track, next_track);
            loc.distance = if next_track.nodes.0 == e_switch {
                overflow_distance.abs()
            } else {
                next_track.length() - overflow_distance.abs()
            };
        }
    }

    pub fn next_track(&self, floc: &mut (Location, Direction)) -> Result<(), LocError> {
        let current_track = self.segments.get(floc.0.track).unwrap(); // CLEAN
        let e_switch = match floc.1 {
            Direction::FacingA => current_track.nodes.0,
            Direction::FacingB => current_track.nodes.1,
        };
        let switch = self.switches.get(e_switch).unwrap(); // CLEAN
        let e_next = match switch.next_segment(floc.0.track) {
            Some(s) => s,
            None => return Err(LocError::NoNeighborSegment),
        };
        let next_track = self.segments.get(e_next).unwrap(); // CLEAN
        floc.0.track = e_next;
        floc.1 = select_direction(floc.1, current_track, next_track);
        floc.0.distance = if next_track.nodes.0 == e_switch {
            0.0
        } else {
            next_track.length()
        };
        Ok(())
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
    dir: Direction,
    current_track: &TrackSegment,
    next_track: &TrackSegment,
) -> Direction {
    if current_track.nodes.0 == next_track.nodes.0 {
        dir.flip()
    } else if current_track.nodes.1 == next_track.nodes.1 {
        dir.flip()
    } else {
        dir
    }
}
