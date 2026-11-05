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
    pub forwards: bool,
    pub landmark: Entity,
    pub train: Entity,
    pub axle: Axle,
}

// fn check_landmarks_passed(moved: On<AxleMoved>, mut commands: Commands, scanner: Scanner) {
//     let AxleMoved {
//         train: e_train,
//         from,
//         to,
//     } = *moved;

//     let passed_landmarks = match scanner.scan(from, to) {
//         Ok(p) => {
//             if p.len() > 0 {
//                 p
//             } else {
//                 return;
//             }
//         }
//         Err(e) => {
//             eprintln!("{e}");
//             return;
//         }
//     };

//     for (e_landmark, forwards) in passed_landmarks {
//         commands.trigger(LandmarkPassed {
//             forwards: forwards,
//             landmark: e_landmark,
//             train: e_train,
//         });
//     }
// }
