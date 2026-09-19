use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    landmark::Landmark,
    loc::FacingLocation,
    zone::{AxleCounter, Zone, block::Block},
};

#[derive(Resource, Default, Debug)]
pub struct ZoneStore {
    pub zones: Vec<Entity>,
}

#[derive(SystemParam)]
pub struct ZoneBuilder<'w, 's> {
    store: ResMut<'w, ZoneStore>,
    commands: Commands<'w, 's>,
}

#[derive(Event)]
pub struct ZonesBuilt;

impl<'w, 's> ZoneBuilder<'w, 's> {
    pub fn new(&mut self) -> Entity {
        let e_zone = self
            .commands
            .spawn((GlobalTransform::default(), Zone::default()))
            .id();
        self.store.zones.push(e_zone);

        e_zone
    }

    pub fn add_counter(&mut self, zone: Entity, floc: FacingLocation) {
        let counter = AxleCounter { zone: zone };
        let e_counter = self.commands.spawn((counter, Landmark, floc)).id();

        self.commands.entity(zone).add_child(e_counter);
    }

    pub fn block(&mut self, zone: Entity) {
        self.commands.entity(zone).insert(Block);
    }

    pub fn flush(&mut self) {
        self.commands.trigger(ZonesBuilt);
    }
}
