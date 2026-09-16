use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    TrackStore,
    landmark::{Landmark, LandmarkPassed},
    loc::{FacingLocation, locator::Locator},
};

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

#[derive(Event)]
pub struct ZoneStatusUpdate {
    zone: Entity,
    status: ZoneStatus,
}

pub enum ZoneStatus {
    Clear,
    Occupied,
}

#[derive(Component, Default)]
pub struct Zone {
    pub axle_count: usize,
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

    let axle_count = &mut zone.axle_count;

    if forwards {
        *axle_count += 1;
        if *axle_count == 1 {
            commands.trigger(ZoneStatusUpdate {
                zone: e_zone,
                status: ZoneStatus::Occupied,
            });
        }
    } else if *axle_count > 0 {
        *axle_count -= 1;
        if *axle_count == 0 {
            commands.trigger(ZoneStatusUpdate {
                zone: e_zone,
                status: ZoneStatus::Clear,
            });
        }
    }
}
