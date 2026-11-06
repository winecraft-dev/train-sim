use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    landmark::{AxleIntrusion, LandmarkPassed},
    train::axle::Axle,
    zone::{block::BlockPlugin, junction::JunctionPlugin, store::ZoneStore},
};

pub mod block;
pub mod builder;
pub mod junction;
pub mod store;

pub struct AxleCounterPlugin;

impl Plugin for AxleCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlockPlugin)
            .add_plugins(JunctionPlugin)
            .add_systems(PreStartup, ZoneStore::init)
            .add_observer(axle_crossed);
    }
}

#[derive(Component)]
pub struct AxleCounter {
    zone: Entity,
}

#[derive(Component, Default)]
pub struct Zone {
    pub occupation: HashMap<Entity, (Axle, bool)>,
}

#[derive(Debug, Clone, Copy)]
pub enum ZoneStatus {
    Clear,
    Occupied,
}

impl Zone {
    pub fn status(&self) -> ZoneStatus {
        match self.occupation.len() {
            0 => ZoneStatus::Clear,
            _ => ZoneStatus::Occupied,
        }
    }

    pub fn crossed(&mut self, train: Entity, intrusion: AxleIntrusion) -> Option<ZoneStatus> {
        if let Some(first_axle) = self.occupation.get(&train) {
            if entered || axle == *first_axle {
                return None;
            }
            self.occupation.remove(&train);
            Some(ZoneStatus::Clear)
        } else {
            if !entered {
                return None;
            }
            self.occupation.insert(train, axle);
            Some(ZoneStatus::Occupied)
        }
    }
}

#[derive(Event)]
pub struct ZoneUpdate {
    zone: Entity,
    status: ZoneStatus,
}

fn axle_crossed(
    crossed: On<LandmarkPassed>,
    mut commands: Commands,
    counters: Query<&AxleCounter>,
    mut zones: Query<&mut Zone>,
) {
    let LandmarkPassed {
        landmark: e_counter,
        train: e_train,
        intrusion,
    } = *crossed;

    let (e_zone, mut zone) = match counters.get(e_counter) {
        Ok(c) => match zones.get_mut(c.zone) {
            Ok(z) => (c.zone, z),
            Err(_) => return,
        },
        Err(_) => return,
    };

    let res = zone.crossed(e_train, intrusion);
    println!("Zone[{}] crossed: {:?}, {:?}", e_zone, intrusion, res);
}
