use bevy::{platform::collections::HashSet, prelude::*};

use crate::{
    landmark::LandmarkPassed,
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
    pub axles: HashSet<Entity>,
}

#[derive(Debug, Clone, Copy)]
pub enum ZoneStatus {
    Clear,
    Occupied,
}

impl Zone {
    pub fn status(&self) -> ZoneStatus {
        match self.axles.len() {
            0 => ZoneStatus::Clear,
            _ => ZoneStatus::Occupied,
        }
    }

    pub fn crossed(&mut self, axle: Entity, entry: bool) -> Option<ZoneStatus> {
        if entry {
            if self.axles.contains(&axle) {
                None
            } else {
                self.axles.insert(axle);
                Some(ZoneStatus::Occupied)
            }
        } else {
            if self.axles.remove(&axle) {
                Some(ZoneStatus::Clear)
            } else {
                None
            }
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
    let passed = crossed.event();

    let (e_zone, mut zone) = match counters.get(passed.landmark) {
        Ok(c) => match zones.get_mut(c.zone) {
            Ok(z) => (c.zone, z),
            Err(_) => return,
        },
        Err(_) => return,
    };

    let res = zone.crossed(passed.axle, passed.aligned);
    // println!("Zone[{}] crossed: {:?}, {:?}", e_zone, passed.axle, res);
}
