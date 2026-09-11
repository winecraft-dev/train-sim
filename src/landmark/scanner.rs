use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    landmark::{Landmark, store::LandmarkStore},
    loc::{Direction, FacingLocation, Location, cursor::TrackCursor, error::LocError},
};

#[derive(SystemParam)]
pub struct Scanner<'w, 's> {
    cursor: TrackCursor<'w, 's>,
    landmarks: Query<'w, 's, (Entity, &'static Location, &'static Direction), With<Landmark>>,
}

type Passed = (Entity, bool);

impl<'w, 's> Scanner<'w, 's> {
    pub fn scan(
        &self,
        a: FacingLocation,
        b: FacingLocation,
        store: Res<LandmarkStore>,
    ) -> Result<Vec<Passed>, LocError> {
        let mut scan_pos = a;
        let mut passed: Vec<Passed> = Vec::new();

        loop {
            // check all landmarks on the current track
            let same_track = scan_pos.0.track == b.0.track;
            let bound_a = scan_pos.0.distance;

            let e_landmarks = store.landmarks_on(&scan_pos.0.track);
            if e_landmarks.len() > 0 {
                match scan_pos.1 {
                    Direction::FacingA => {
                        let bound_b = if same_track { b.0.distance } else { 0.0 };
                        for e_landmark in e_landmarks.iter().rev() {
                            let landmark = self.landmarks.get(*e_landmark).unwrap(); // CLEAN UP
                            let ld = landmark.1.distance;
                            if bound_b <= ld && ld <= bound_a {
                                let forward = *landmark.2 == Direction::FacingA;
                                passed.push((*e_landmark, forward));
                            }
                        }
                    }
                    Direction::FacingB => {
                        let bound_b = if same_track { b.0.distance } else { f32::MAX };
                        for e_landmark in e_landmarks.iter() {
                            let landmark = self.landmarks.get(*e_landmark).unwrap();
                            let ld = landmark.1.distance;
                            if bound_b >= ld && ld >= bound_a {
                                let forward = *landmark.2 == Direction::FacingB;
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
