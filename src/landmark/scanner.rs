use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, BrokenLandmarkReference},
    landmark::{Landmark, store::LandmarkStore},
    loc::{Dir, Loc, traverser::Traverser},
};

#[derive(SystemParam)]
pub struct Scanner<'w, 's> {
    cursor: Traverser<'w, 's>,
    landmarks: Query<'w, 's, (Entity, &'static Loc), With<Landmark>>,
    store: Res<'w, LandmarkStore>,
}

type Passed = (Entity, bool);

impl<'w, 's> Scanner<'w, 's> {
    pub fn scan(&self, a: Loc, b: Loc) -> Result<Vec<Passed>, SimError> {
        let mut scan_pos = a;
        let mut passed: Vec<Passed> = Vec::new();

        loop {
            // check all landmarks on the current track
            let same_track = scan_pos.track == b.track;
            let bound_a = scan_pos.distance;

            let e_landmarks = self.store.landmarks_on(&scan_pos.track);
            if e_landmarks.len() > 0 {
                match scan_pos.dir {
                    Dir::A => {
                        let bound_b = if same_track { b.distance } else { 0.0 };
                        for e_landmark in e_landmarks.iter().rev() {
                            let landmark = match self.landmarks.get(*e_landmark) {
                                Ok(l) => l,
                                Err(_) => return Err(BrokenLandmarkReference(*e_landmark)),
                            };
                            let ld = landmark.1.distance;
                            if bound_b <= ld && ld <= bound_a {
                                let forward = landmark.1.dir == Dir::A;
                                passed.push((*e_landmark, forward));
                            }
                        }
                    }
                    Dir::B => {
                        let bound_b = if same_track { b.distance } else { f32::MAX };
                        for e_landmark in e_landmarks.iter() {
                            let landmark = match self.landmarks.get(*e_landmark) {
                                Ok(l) => l,
                                Err(_) => return Err(BrokenLandmarkReference(*e_landmark)),
                            };
                            let ld = landmark.1.distance;
                            if bound_b >= ld && ld >= bound_a {
                                let forward = landmark.1.dir == Dir::B;
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
