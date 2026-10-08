use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    control::ClickTarget,
    landmark::Landmark,
    loader::{self, wire},
    loc::{FLoc, cursor::TrackCursor},
    signal::{Aspect, ObservableBound, Signal},
    track::builder::TrackStore,
    zone::{
        AxleCounter, Zone,
        block::Block,
        error::ZoneError,
        junction::{Junction, SwitchJunctionLookup},
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
    junction_lookup: ResMut<'w, SwitchJunctionLookup>,
    commands: Commands<'w, 's>,
    cursor: TrackCursor<'w, 's>,
    track_store: Res<'w, TrackStore>,
}

#[derive(Event)]
pub struct ZonesBuilt;

const DEFAULT_SIGNAL_DISTANCE: f32 = 50.0;
const DEFAULT_OBS_DISTANCE: f32 = 35.0;

impl<'w, 's> ZoneBuilder<'w, 's> {
    pub fn from_wire(&mut self, zone: &wire::Zone) -> Result<Entity, ZoneError> {
        let e_zone = self
            .commands
            .spawn((GlobalTransform::default(), Zone::default()))
            .id();

        // let e_signals = self.spawn_signals(e_zone, zone);

        Ok(Entity::from_bits(0))
    }

    // fn setup_block(&mut self, zone: Entity, signal: Entity) {
    //     self.commands.entity(zone).insert(Block {
    //         entry_signal: signal,
    //     });
    //     self.store.push(zone);
    // }

    // fn setup_junction(
    //     &mut self,
    //     zone: Entity,
    //     entry_n: usize,
    //     exit_n: usize,
    //     e_signals: Vec<Entity>,
    //     e_switches: Vec<Entity>,
    // ) {
    //     let junction = match (entry_n, exit_n) {
    //         (1, 2) => Junction::Split1_2 {
    //             signal: e_signals[0],
    //         },
    //         (2, 1) => {
    //             self.junction_lookup.insert(e_switches[0], zone);
    //             Junction::Merge2_1 {
    //                 signals: [e_signals[0], e_signals[1]],
    //                 control: 0,
    //             }
    //         }
    //         _ => todo!(),
    //     };
    //     self.commands.entity(zone).insert(junction);
    // }

    // fn spawn_entries(
    //     &mut self,
    //     parent: Entity,
    //     zone: wire::Zone,
    // ) -> Result<Vec<Entity>, ZoneError> {
    //     let mut e_signals: Vec<Entity> = Vec::new();
    //     let mut e_entrys: Vec<Entity> = Vec::new(); // misspelled for alignment
    //     for entry_loc in zone.entry_locs.iter() {
    //         let entry_loc: FLoc =
    //         let signal_loc = match self.cursor.traverse(*entry_loc, signal_distance) {
    //             Ok(loc) => loc,
    //             Err(e) => return Err(ZoneError::InvalidSignalLoc(e)),
    //         };
    //         let obv_loc = match self.cursor.traverse(signal_loc, observable_distance) {
    //             Ok(loc) => loc,
    //             Err(e) => return Err(ZoneError::InvalidObservableLoc(e)),
    //         };

    //         let e_signal = self.spawn_signal(signal_loc, obv_loc);
    //         let e_entry = self
    //             .commands
    //             .spawn((AxleCounter { zone }, Landmark, *entry_loc))
    //             .id();
    //         e_signals.push(e_signal);
    //         e_entrys.push(e_entry);
    //     }

    //     self.commands
    //         .entity(zone)
    //         .add_children(&e_entrys)
    //         .add_children(&e_signals);

    //     Ok(e_signals)
    // }

    // fn spawn_exits(&mut self, zone: Entity, locs: Vec<FLoc>) -> usize {
    //     let mut e_exits: Vec<Entity> = Vec::new();
    //     for exit_loc in locs.iter() {
    //         let e_exit = self
    //             .commands
    //             .spawn((AxleCounter { zone }, Landmark, *exit_loc))
    //             .id();
    //         e_exits.push(e_exit);
    //     }

    //     self.commands.entity(zone).add_children(&e_exits);
    //     e_exits.len()
    // }

    // fn fetch_switches(&mut self, u_switches: Vec<usize>) -> Vec<Entity> {
    //     let mut e_switches: Vec<Entity> = Vec::new();
    //     for u_switch in u_switches {
    //         let e_switch = self.track_store.nodes[u_switch];
    //         e_switches.push(e_switch);
    //     }
    //     e_switches
    // }

    // fn spawn_signal(&mut self, signal_loc: FLoc, observable_loc: FLoc) -> Entity {
    //     let e_signal = self
    //         .commands
    //         .spawn((
    //             Signal {
    //                 aspect: Aspect::default(),
    //             },
    //             ClickTarget,
    //             signal_loc,
    //         ))
    //         .id();

    //     let e_obv = self
    //         .commands
    //         .spawn((
    //             ObservableBound { signal: e_signal },
    //             Landmark,
    //             observable_loc,
    //         ))
    //         .id();

    //     self.commands.entity(e_signal).add_child(e_obv);

    //     e_signal
    // }

    pub fn built(&mut self) {
        self.commands.trigger(ZonesBuilt);
    }
}
