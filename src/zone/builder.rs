use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    landmark::Landmark,
    loc::{FacingLocation, cursor::TrackCursor},
    zone::{AxleCounter, Zone, block::Block, junction::Junction},
};

#[derive(Resource, Default, Debug)]
pub struct ZoneStore {
    pub zones: Vec<Entity>,
}

#[derive(SystemParam)]
pub struct ZoneBuilder<'w, 's> {
    store: ResMut<'w, ZoneStore>,
    commands: Commands<'w, 's>,
    cursor: TrackCursor<'w, 's>,
}

#[derive(Default)]
pub struct ZoneConstructor {
    entry_locs: Vec<FacingLocation>,
    exit_locs: Vec<FacingLocation>,
}

impl ZoneConstructor {
    pub fn new() -> Self {
        Self { ..default() }
    }

    pub fn add_entry(mut self, floc: FacingLocation) -> Self {
        self.entry_locs.push(floc);
        self
    }

    pub fn add_exit(mut self, floc: FacingLocation) -> Self {
        self.exit_locs.push(floc);
        self
    }
}

#[derive(Event)]
pub struct ZonesBuilt;

impl<'w, 's> ZoneBuilder<'w, 's> {
    pub fn new(&mut self, zone: ZoneConstructor) -> Entity {
        let e_zone = self
            .commands
            .spawn((GlobalTransform::default(), Zone::default()))
            .id();

        let mut e_entrys: Vec<Entity> = Vec::new(); // misspelled for alignment
        let mut e_exits: Vec<Entity> = Vec::new();

        // check for if junction or block given how many entrys/exits
        for entry_loc in zone.entry_locs.iter() {
            let e_entry = self
                .commands
                .spawn((AxleCounter { zone: e_zone }, Landmark, *entry_loc))
                .id();
            e_entrys.push(e_entry);
        }

        for exit_loc in zone.exit_locs.iter() {
            let e_exit = self
                .commands
                .spawn((AxleCounter { zone: e_zone }, Landmark, *exit_loc))
                .id();
            e_exits.push(e_exit);
        }

        let signal_loc = self.cursor.traverse(zone.entry_locs[0], -15.0).unwrap();

        e_zone
    }

    pub fn add_counter(&mut self, zone: Entity, floc: FacingLocation) {
        let counter = AxleCounter { zone: zone };
        let e_counter = self.commands.spawn((counter, Landmark, floc)).id();

        self.commands.entity(zone).add_child(e_counter);
    }
}
