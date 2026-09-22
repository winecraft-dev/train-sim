use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    landmark::Landmark,
    loc::{FacingLocation, cursor::TrackCursor},
    signal::builder::SignalBuilder,
    track::builder::TrackStore,
    zone::{
        AxleCounter, Zone,
        block::Block,
        junction::{Junction, JunctionVariant},
    },
};

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct ZoneStore(Vec<Entity>);

pub fn init_zone_store(mut commands: Commands) {
    commands.insert_resource(ZoneStore::default());
}

#[derive(SystemParam)]
pub struct ZoneBuilder<'w, 's> {
    store: ResMut<'w, ZoneStore>,
    commands: Commands<'w, 's>,
    cursor: TrackCursor<'w, 's>,
    track_store: Res<'w, TrackStore>,
    signal_builder: SignalBuilder<'w, 's>,
}

#[derive(Default)]
pub struct ZoneConstructor {
    entry_locs: Vec<FacingLocation>,
    exit_locs: Vec<FacingLocation>,
    switches: Vec<usize>,
}

impl ZoneConstructor {
    pub fn new() -> Self {
        Self { ..default() }
    }

    pub fn with_entry(mut self, floc: FacingLocation) -> Self {
        self.entry_locs.push(floc);
        self
    }

    pub fn with_exit(mut self, floc: FacingLocation) -> Self {
        self.exit_locs.push(floc);
        self
    }

    pub fn with_switch(mut self, switch: usize) -> Self {
        self.switches.push(switch);
        self
    }

    pub fn build(self, builder: &mut ZoneBuilder) {
        builder.new(self);
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

        let (e_signals, n_entrys) = self.spawn_entries(e_zone, zone.entry_locs);
        let n_exits = self.spawn_exits(e_zone, zone.exit_locs);
        let e_switches = self.fetch_switches(zone.switches);

        // count entrys to exits to gather the type of Block/Junction
        if let (1, 1) = (n_entrys, n_exits) {
            self.commands.entity(e_zone).insert(Block {
                entry_signal: e_signals[0],
            });
            self.store.push(e_zone);
            return e_zone;
        } else {
            let junction = match (n_entrys, n_exits) {
                (1, 2) => Junction {
                    variant: JunctionVariant::Split1_2 {
                        signal: e_signals[0],
                        switch: e_switches[0],
                    },
                },
                (2, 1) => Junction {
                    variant: JunctionVariant::Merge2_1 {
                        signals: [e_signals[0], e_signals[1]],
                        switch: e_switches[0],
                    },
                },
                _ => todo!(),
            };
            println!("{:?}", junction);
            self.commands.entity(e_zone).insert(junction);
            println!("A problem");
        }

        self.store.push(e_zone);
        e_zone
    }

    fn spawn_entries(&mut self, zone: Entity, locs: Vec<FacingLocation>) -> (Vec<Entity>, usize) {
        let mut e_signals: Vec<Entity> = Vec::new();
        let mut e_entrys: Vec<Entity> = Vec::new(); // misspelled for alignment
        for entry_loc in locs.iter() {
            let signal_loc = self.cursor.traverse(*entry_loc, -15.0).unwrap();
            let e_signal = self.signal_builder.new(signal_loc, -50.0);
            let e_entry = self
                .commands
                .spawn((AxleCounter { zone }, Landmark, *entry_loc))
                .id();
            e_signals.push(e_signal);
            e_entrys.push(e_entry);
        }

        self.commands.entity(zone).add_children(&e_entrys);
        //     .add_children(&e_signals); // BREAKS THINGS, Don't include for now but maybe we need to bring signal building into
        // here

        (e_signals, e_entrys.len())
    }

    fn spawn_exits(&mut self, zone: Entity, locs: Vec<FacingLocation>) -> usize {
        let mut e_exits: Vec<Entity> = Vec::new();
        for exit_loc in locs.iter() {
            let e_exit = self
                .commands
                .spawn((AxleCounter { zone }, Landmark, *exit_loc))
                .id();
            e_exits.push(e_exit);
        }

        self.commands.entity(zone).add_children(&e_exits);
        e_exits.len()
    }

    fn fetch_switches(&mut self, u_switches: Vec<usize>) -> Vec<Entity> {
        let mut e_switches: Vec<Entity> = Vec::new();
        for u_switch in u_switches {
            let e_switch = self.track_store.nodes[u_switch];
            e_switches.push(e_switch);
        }
        e_switches
    }

    pub fn done(&mut self) {
        self.commands.trigger(ZonesBuilt);
    }
}
