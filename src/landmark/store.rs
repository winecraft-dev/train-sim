use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    landmark::{IndexedLandmark, Landmark},
    loc::{Direction, Location},
};

pub struct LandmarksStorePlugin;

impl Plugin for LandmarksStorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_store)
            .add_systems(Update, update_store);
    }
}

#[derive(Resource, Default, DerefMut, Deref, Debug)]
pub struct LandmarkStore(HashMap<Entity, Vec<(Entity, f32)>>);

impl LandmarkStore {
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

fn init_store(mut commands: Commands) {
    let store = LandmarkStore::default();

    commands.insert_resource(store);
}

fn update_store(
    mut commands: Commands,
    mut store: ResMut<LandmarkStore>,
    landmarks: Query<(Entity, &Location, &Direction), (With<Landmark>, Without<IndexedLandmark>)>,
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
