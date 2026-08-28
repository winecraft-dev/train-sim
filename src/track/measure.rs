use bevy::prelude::*;

use crate::{
    switch::TrackSwitch,
    track::{
        TrackSegment,
        cursor::select_direction,
        error::TrackError,
        loc::{Direction, Location},
    },
};

const MAX_JUMPS: usize = 10;

// pub fn distance(
//     a: Location,
//     b: Location,
//     segments: Query<&TrackSegment>,
//     switches: Query<&TrackSwitch>,
// ) -> Result<f32, TrackError> {
//     let mut jumps: usize = 0;
//     let mut distance: f32 = 0.0;
//     let mut cursor = a.clone();
//     loop {
//         if jumps > MAX_JUMPS {
//             return Err(TrackError::ExceedsMaxJumps);
//         }
//         // check if on the same track currently
//         if cursor.track == b.track {
//             let delta = b.distance - cursor.distance;
//             match cursor.direction {
//                 Direction::FacingA => {
//                     if delta < 0.0 {
//                         distance += delta.abs();
//                         break;
//                     }
//                 }
//                 Direction::FacingB => {
//                     if delta >= 0.0 {
//                         distance += delta.abs();
//                         break;
//                     }
//                 }
//             }
//         }

//         // traverse to next segment
//         let current_track = segments.get(cursor.track).unwrap(); // CLEAN
//         let (e_switch, delta) = match cursor.direction {
//             Direction::FacingA => (current_track.nodes.0, cursor.distance),
//             Direction::FacingB => (
//                 current_track.nodes.1,
//                 current_track.length() - cursor.distance,
//             ),
//         };
//         let switch = switches.get(e_switch).unwrap(); // CLEAN
//         let e_next = match switch.next_segment(cursor.track) {
//             Some(s) => s,
//             None => return Err(TrackError::NoNeighborSegment),
//         };
//         let next_track = segments.get(e_next).unwrap(); // CLEAN

//         distance += delta.abs();
//         cursor.track = e_next;
//         select_direction(current_track, next_track);
//         cursor.distance = if next_track.nodes.0 == e_switch {
//             0.0
//         } else {
//             next_track.length()
//         };

//         jumps += 1;
//     }
//     Ok(distance)
// }
