use std::cmp::Ordering;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    landmark::{IndexedLandmark, Landmark},
    loc::{Direction, Location},
};

#[derive(Event)]
pub struct LandmarksUpdated;

#[derive(Resource, Default, DerefMut, Deref)]
pub struct LandmarkStore(HashMap<Entity, Vec<Entity>>);

pub fn setup_store(
    mut commands: Commands,
    landmarks: Query<(Entity, &Location, &Direction), (With<Landmark>, Without<IndexedLandmark>)>,
) {
    let mut store = LandmarkStore::default();

    for (e_landmark, loc, _) in landmarks {
        let e_track = loc.track;
        match store.get_mut(&e_track) {
            Some(l) => {
                l.push(e_landmark);
            }
            None => {
                let mut l: Vec<Entity> = Vec::new();
                l.push(e_landmark);
                store.insert(e_track, l);
            }
        };
    }
    for (_, l) in store.iter_mut() {
        l.sort_by(|a, b| compare_landmarks(*a, *b, landmarks));
    }

    commands.insert_resource(store);
}

fn compare_landmarks(
    a: Entity,
    b: Entity,
    landmarks: Query<(Entity, &Location, &Direction), (With<Landmark>, Without<IndexedLandmark>)>,
) -> Ordering {
    let la = landmarks.get(a).unwrap();
    let lb = landmarks.get(b).unwrap();

    la.1.distance.total_cmp(&lb.1.distance)
}
