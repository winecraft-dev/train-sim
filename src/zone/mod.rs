use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    landmark::{Landmark, LandmarkPassed},
    loc::FacingLocation,
    zone::block::Block,
};

pub mod block;
pub mod junction;

pub struct AxleCounterPlugin;

impl Plugin for AxleCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(axle_crossed);
    }
}

#[derive(Component)]
pub struct AxleCounter {
    zone: Entity,
}

#[derive(Component, Default)]
pub struct Zone {
    pub count: usize,
}

pub enum ZoneStatus {
    Clear,
    Occupied,
}

impl Zone {
    fn crossed(&mut self, forwards: bool) -> Option<ZoneStatus> {
        if forwards {
            self.count += 1;
            if self.count == 1 {
                return Some(ZoneStatus::Occupied);
            }
        } else if self.count > 0 {
            self.count -= 1;
            if self.count == 0 {
                return Some(ZoneStatus::Clear);
            }
        }
        None
    }

    fn status(&self) -> ZoneStatus {
        match self.count {
            0 => ZoneStatus::Clear,
            _ => ZoneStatus::Occupied,
        }
    }
}

#[derive(SystemParam)]
pub struct ZoneBuilder<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'w, 's> ZoneBuilder<'w, 's> {
    pub fn new(&mut self) -> Entity {
        self.commands
            .spawn((GlobalTransform::default(), Zone::default()))
            .id()
    }

    pub fn add_counter(&mut self, zone: Entity, floc: FacingLocation) {
        let counter = AxleCounter { zone: zone };
        let e_counter = self.commands.spawn((counter, Landmark, floc)).id();

        self.commands.entity(zone).add_child(e_counter);
    }

    pub fn block(&mut self, zone: Entity) {
        self.commands.entity(zone).insert(Block);
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
        forwards,
        landmark: e_counter,
        train: _,
    } = *crossed;

    let counter = match counters.get(e_counter) {
        Ok(c) => c,
        Err(_) => return,
    };

    let e_zone = counter.zone;
    let mut zone = match zones.get_mut(e_zone) {
        Ok(z) => z,
        Err(_) => return,
    };

    match zone.crossed(forwards) {
        Some(s) => commands.trigger(ZoneUpdate {
            zone: e_zone,
            status: s,
        }),
        None => return,
    }
}
