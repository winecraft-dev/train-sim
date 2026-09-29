use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    landmark::{
        Landmark,
        error::LandmarkError::{self, BrokenLandmarkReference, CursorError},
        store::LandmarkStore,
    },
    loc::{Dir, FacingLocation, Loc, cursor::TrackCursor},
};

#[derive(SystemParam)]
pub struct Scanner<'w, 's> {
    cursor: TrackCursor<'w, 's>,
    landmarks: Query<'w, 's, (Entity, &'static Loc, &'static Dir), With<Landmark>>,
}

type Passed = (Entity, bool);

impl<'w, 's> Scanner<'w, 's> {
    pub fn scan(
        &self,
        a: FacingLocation,
        b: FacingLocation,
        store: Res<LandmarkStore>,
    ) -> Result<Vec<Passed>, LandmarkError> {
        let mut scan_pos = a;
        let mut passed: Vec<Passed> = Vec::new();

        loop {
            // check all landmarks on the current track
            let same_track = scan_pos.0.track == b.0.track;
            let bound_a = scan_pos.0.distance;

            let e_landmarks = store.landmarks_on(&scan_pos.0.track);
            if e_landmarks.len() > 0 {
                match scan_pos.1 {
                    Dir::ToA => {
                        let bound_b = if same_track { b.0.distance } else { 0.0 };
                        for e_landmark in e_landmarks.iter().rev() {
                            let landmark = match self.landmarks.get(*e_landmark) {
                                Ok(l) => l,
                                Err(_) => return Err(BrokenLandmarkReference(*e_landmark)),
                            };
                            let ld = landmark.1.distance;
                            if bound_b <= ld && ld <= bound_a {
                                let forward = *landmark.2 == Dir::ToA;
                                passed.push((*e_landmark, forward));
                            }
                        }
                    }
                    Dir::ToB => {
                        let bound_b = if same_track { b.0.distance } else { f32::MAX };
                        for e_landmark in e_landmarks.iter() {
                            let landmark = match self.landmarks.get(*e_landmark) {
                                Ok(l) => l,
                                Err(_) => return Err(BrokenLandmarkReference(*e_landmark)),
                            };
                            let ld = landmark.1.distance;
                            if bound_b >= ld && ld >= bound_a {
                                let forward = *landmark.2 == Dir::ToB;
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
            if let Err(e) = self.cursor.next_track(&mut scan_pos) {
                return Err(CursorError(e));
            };
        }
        Ok(passed)
    }
}
