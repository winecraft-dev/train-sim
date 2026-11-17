use bevy::prelude::*;

pub mod scanner;
pub mod store;

use crate::landmark::store::LandmarkStore;

pub struct LandmarkPlugin;

impl Plugin for LandmarkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, LandmarkStore::init)
            .add_systems(Update, LandmarkStore::update);
    }
}

#[derive(Component)]
pub struct Landmark;

#[derive(Component)]
pub struct IndexedLandmark;

#[derive(Event)]
pub struct LandmarkPassed {
    pub landmark: Entity,
    pub train: Entity,
    pub axle: Entity,
    pub aligned: bool,
}

impl LandmarkPassed {
    pub fn primary(&self) -> bool {
        self.train == self.axle
    }
}
