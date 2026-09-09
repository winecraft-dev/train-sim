use bevy::{ecs::system::SystemParam, prelude::*};

pub mod store;

use crate::{
    landmark::store::{LandmarkStore, setup_store},
    loc::{Direction, FacingLocation, Location, cursor::TrackCursor, error::LocError},
    train::axle::AxleMoved,
};

pub struct LandmarkPlugin;

impl Plugin for LandmarkPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(check_landmarks_passed)
            .add_observer(setup_store);
    }
}

#[derive(Component)]
pub struct Landmark;

#[derive(Event)]
pub struct LandmarkPassed {
    pub forwards: bool,
    pub landmark: Entity,
    pub train: Entity,
}

fn check_landmarks_passed(
    moved: On<AxleMoved>,
    mut commands: Commands,
    store: Res<LandmarkStore>,
    scanner: Scanner,
) {
    let AxleMoved {
        train: e_train,
        from,
        to,
    } = *moved;

    let passed_landmarks = match scanner.scan(from, to, store) {
        Ok(p) => {
            if p.len() > 0 {
                p
            } else {
                return;
            }
        }
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    println!("Passed {passed_landmarks:?}");

    for (e_landmark, forwards) in passed_landmarks {
        commands.trigger(LandmarkPassed {
            forwards: forwards,
            landmark: e_landmark,
            train: e_train,
        });
    }
}

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

            let e_landmarks = match store.get(&scan_pos.0.track) {
                Some(l) => l,
                None => &Vec::new(),
            };

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
