use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    loc::{Direction, FacingLocation, Location, error::LocError},
    switch::TrackSwitch,
    track::TrackSegment,
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

    pub fn passed(
        &self,
        from: FacingLocation,
        to: FacingLocation,
        check: Location,
    ) -> Result<bool, LocError> {
        let mut c_from = from;
        let mut c_to = to;
        c_to.1 = c_to.1.flip();
        loop {
            // all on the same track
            if c_from.0.track == check.track && c_to.0.track == check.track {
                let a = c_from.0.distance;
                let b = c_to.0.distance;
                let x = check.distance;

                match c_from.1 {
                    Direction::FacingB => {
                        if a <= x && x <= b {
                            return Ok(true);
                        }
                    }
                    Direction::FacingA => {
                        if a >= x && x >= b {
                            return Ok(true);
                        }
                    }
                };
                return Ok(false);
            } else if c_from.0.track == c_to.0.track {
                return Ok(false);
            }

            if c_from.0.track != check.track {
                // c_from traverses forwards
                let current_track = self.segments.get(c_from.0.track).unwrap(); // CLEAN
                let e_switch = match c_from.1 {
                    Direction::FacingA => current_track.nodes.0,
                    Direction::FacingB => current_track.nodes.1,
                };
                let switch = self.switches.get(e_switch).unwrap(); // CLEAN
                let e_next = match switch.next_segment(c_from.0.track) {
                    Some(s) => s,
                    None => return Err(LocError::NoNeighborSegment),
                };
                let next_track = self.segments.get(e_next).unwrap(); // CLEAN

                c_from.0.track = e_next;
                c_from.1 = select_direction(c_from.1, current_track, next_track);
                c_from.0.distance = if next_track.nodes.0 == e_switch {
                    0.0
                } else {
                    next_track.length()
                };
            } else if c_to.0.track != check.track {
                // c_to traverses backwards
                let current_track = self.segments.get(c_to.0.track).unwrap(); // CLEAN
                let e_switch = match c_to.1 {
                    Direction::FacingA => current_track.nodes.0,
                    Direction::FacingB => current_track.nodes.1,
                };
                let switch = self.switches.get(e_switch).unwrap(); // CLEAN
                let e_next = match switch.next_segment(c_to.0.track) {
                    Some(s) => s,
                    None => return Err(LocError::NoNeighborSegment),
                };
                let next_track = self.segments.get(e_next).unwrap(); // CLEAN

                c_to.0.track = e_next;
                c_to.1 = select_direction(c_to.1, current_track, next_track);
                c_to.0.distance = if next_track.nodes.0 == e_switch {
                    0.0
                } else {
                    next_track.length()
                };
            }
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
