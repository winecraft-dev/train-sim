use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    landmark::{IndexedLandmark, Landmark},
    loc::{Dir, Loc},
};

#[derive(Resource, Default, DerefMut, Deref, Debug)]
pub struct LandmarkStore(HashMap<Entity, Vec<(Entity, f32)>>);

impl LandmarkStore {
    pub fn init(mut commands: Commands) {
        commands.insert_resource(Self::default());
    }

    pub fn reset(mut store: ResMut<Self>) {
        store.clear();
    }

    pub fn update(
        mut commands: Commands,
        mut store: ResMut<Self>,
        landmarks: Query<(Entity, &Loc, &Dir), (With<Landmark>, Without<IndexedLandmark>)>,
    ) {
        for (e_landmark, loc, _) in landmarks {
            let e_track = loc.track;
            match store.get_mut(&e_track) {
                Some(l) => {
                    l.push((e_landmark, loc.distance));
                }
                None => {
                    let mut l: Vec<(Entity, f32)> = Vec::new();
                    l.push((e_landmark, loc.distance));
                    store.insert(e_track, l);
                }
            };
            commands.entity(e_landmark).insert(IndexedLandmark);
        }
        for (_, l) in store.iter_mut() {
            l.sort_by(|a, b| a.1.total_cmp(&b.1));
        }
    }

    pub fn landmarks_on(&self, track: &Entity) -> Vec<Entity> {
        match self.get(track) {
            Some(vec) => vec
                .iter()
                .map(|(e_landmark, _)| *e_landmark)
                .collect::<Vec<Entity>>(),
            None => Vec::new(),
        }
    }
}
