use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    control::ClickTarget,
    error::SimError::{self, InvalidObservableBoundLoc, InvalidSignalLoc},
    landmark::Landmark,
    loader::wire,
    loc::{FLoc, cursor::TrackCursor, locator::Locator},
    signal::{ObservableBound, Signal},
    track::builder::TrackStore,
    zone::{
        AxleCounter, Zone,
        block::Block,
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
    locator: Locator<'w, 's>,
    track_store: Res<'w, TrackStore>,
}

#[derive(Event)]
pub struct ZonesBuilt;

const DEFAULT_SIGNAL_DISTANCE: f32 = -15.0;
const DEFAULT_OBS_DISTANCE: f32 = -50.0;

impl<'w, 's> ZoneBuilder<'w, 's> {
    pub fn from_wire(&mut self, zone: &wire::Zone) -> Result<Entity, SimError> {
        let e_zone = self
            .commands
            .spawn((GlobalTransform::default(), Zone::default()))
            .id();

        let e_signals = self.spawn_signals(e_zone, zone)?;
        let e_switches = self.fetch_switches(zone)?;
        let exit_n = self.spawn_exits(e_zone, zone)?;

        self.classify_zone(e_zone, e_signals, e_switches, exit_n);
        self.store.push(e_zone);

        Ok(e_zone)
    }

    fn classify_zone(
        &mut self,
        zone: Entity,
        e_signals: Vec<Entity>,
        e_switches: Vec<Entity>,
        exit_n: usize,
    ) {
        let entry_n = e_signals.len();

        if (entry_n, exit_n) == (1, 1) {
            self.commands.entity(zone).insert(Block {
                entry_signal: e_signals[0],
            });
            return;
        }
        let junction = match (entry_n, exit_n) {
            (1, 2) => Junction::Split1_2 {
                signal: e_signals[0],
            },
            (2, 1) => {
                self.junction_lookup.insert(e_switches[0], zone);
                Junction::Merge2_1 {
                    signals: [e_signals[0], e_signals[1]],
                    control: 0,
                }
            }
            _ => todo!(),
        };
        self.commands.entity(zone).insert(junction);
    }

    pub fn spawn_signals(
        &mut self,
        e_zone: Entity,
        zone: &wire::Zone,
    ) -> Result<Vec<Entity>, SimError> {
        let mut e_signals: Vec<Entity> = Vec::new();
        let mut e_entrys: Vec<Entity> = Vec::new();

        for entry_loc in zone.entrys.iter() {
            let entry_loc = self.locator.convert(*entry_loc)?;
            let e_signal = self.spawn_signal(zone, entry_loc)?;
            let e_entry = self
                .commands
                .spawn((AxleCounter { zone: e_zone }, Landmark, entry_loc))
                .id();

            e_signals.push(e_signal);
            e_entrys.push(e_entry);
        }

        self.commands
            .entity(e_zone)
            .add_children(&e_signals)
            .add_children(&e_entrys);

        Ok(e_signals)
    }

    pub fn spawn_signal(&mut self, zone: &wire::Zone, entry_loc: FLoc) -> Result<Entity, SimError> {
        let signal_distance = match zone.signal_distance {
            Some(d) => d,
            None => DEFAULT_SIGNAL_DISTANCE,
        };
        let obv_distance = match zone.observable_distance {
            Some(d) => d,
            None => DEFAULT_OBS_DISTANCE,
        };
        let signal_loc = match self.cursor.traverse(entry_loc, signal_distance) {
            Ok(loc) => loc,
            Err(e) => return Err(InvalidSignalLoc(Box::new(e))),
        };
        let obv_loc = match self.cursor.traverse(signal_loc, obv_distance) {
            Ok(loc) => loc,
            Err(e) => return Err(InvalidObservableBoundLoc(Box::new(e))),
        };

        let e_signal = self
            .commands
            .spawn((Signal::default(), ClickTarget, signal_loc))
            .id();

        let e_obv = self
            .commands
            .spawn((ObservableBound { signal: e_signal }, Landmark, obv_loc))
            .id();

        self.commands.entity(e_signal).add_child(e_obv);

        Ok(e_signal)
    }

    fn spawn_exits(&mut self, e_zone: Entity, zone: &wire::Zone) -> Result<usize, SimError> {
        let mut e_exits: Vec<Entity> = Vec::new();
        for exit_loc in zone.exits.iter() {
            let exit_loc = self.locator.convert(*exit_loc)?;
            let e_exit = self
                .commands
                .spawn((AxleCounter { zone: e_zone }, Landmark, exit_loc))
                .id();
            e_exits.push(e_exit);
        }

        self.commands.entity(e_zone).add_children(&e_exits);
        Ok(e_exits.len())
    }

    fn fetch_switches(&mut self, zone: &wire::Zone) -> Result<Vec<Entity>, SimError> {
        let mut e_switches: Vec<Entity> = Vec::new();
        let Some(ref switches) = zone.switches else {
            return Ok(e_switches);
        };
        for i in switches.iter() {
            let e_switch = self.track_store.node(*i)?;
            e_switches.push(e_switch);
        }
        Ok(e_switches)
    }

    pub fn built(&mut self) {
        self.commands.trigger(ZonesBuilt);
    }
}
