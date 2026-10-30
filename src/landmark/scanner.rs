use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, BrokenLandmarkReference},
    landmark::{Landmark, store::LandmarkStore},
    loc::{Dir, FLoc, Loc, cursor::TrackCursor},
};

#[derive(SystemParam)]
pub struct Scanner<'w, 's> {
    cursor: TrackCursor<'w, 's>,
    landmarks: Query<'w, 's, (Entity, &'static Loc, &'static Dir), With<Landmark>>,
    store: Res<'w, LandmarkStore>,
}

type Passed = (Entity, bool);

impl<'w, 's> Scanner<'w, 's> {
    pub fn scan(&self, a: FLoc, b: FLoc) -> Result<Vec<Passed>, SimError> {
        let mut scan_pos = a;
        let mut passed: Vec<Passed> = Vec::new();

        loop {
            // check all landmarks on the current track
            let same_track = scan_pos.0.track == b.0.track;
            let bound_a = scan_pos.0.distance;

            let e_landmarks = self.store.landmarks_on(&scan_pos.0.track);
            if e_landmarks.len() > 0 {
                match scan_pos.1 {
                    Dir::A => {
                        let bound_b = if same_track { b.0.distance } else { 0.0 };
                        for e_landmark in e_landmarks.iter().rev() {
                            let landmark = match self.landmarks.get(*e_landmark) {
                                Ok(l) => l,
                                Err(_) => return Err(BrokenLandmarkReference(*e_landmark)),
                            };
                            let ld = landmark.1.distance;
                            if bound_b <= ld && ld <= bound_a {
                                let forward = *landmark.2 == Dir::A;
                                passed.push((*e_landmark, forward));
                            }
                        }
                    }
                    Dir::B => {
                        let bound_b = if same_track { b.0.distance } else { f32::MAX };
                        for e_landmark in e_landmarks.iter() {
                            let landmark = match self.landmarks.get(*e_landmark) {
                                Ok(l) => l,
                                Err(_) => return Err(BrokenLandmarkReference(*e_landmark)),
                            };
                            let ld = landmark.1.distance;
                            if bound_b >= ld && ld >= bound_a {
                                let forward = *landmark.2 == Dir::B;
                                passed.push((*e_landmark, forward));
                            }
                        }
                    }
                };
            }
            // decide to either continue to next segment or break
            if same_track {
                break;
            }
            self.cursor.next_track(&mut scan_pos)?;
        }
        Ok(passed)
    }
}
