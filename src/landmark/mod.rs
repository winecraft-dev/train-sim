use bevy::prelude::*;

pub mod scanner;
pub mod store;

use crate::{landmark::store::LandmarkStore, train::axle::Axle};

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
    pub intrusion: AxleIntrusion,
}

#[derive(Debug, Copy, Clone)]
pub struct AxleIntrusion {
    pub axle: Axle,
    pub aligned: bool,
    pub forwards: bool,
}

impl AxleIntrusion {
    pub fn train_entry(&self) -> bool {
        match self.axle {
            Axle::Front => self.forwards == true,
            Axle::Rear => self.forwards == false,
        }
    }
}
