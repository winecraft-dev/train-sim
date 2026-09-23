use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    signal::{Aspect, control::SignalControl},
    track::switch::SwitchUpdate,
    zone::{Zone, ZoneStatus, ZoneUpdate},
};

pub struct JunctionPlugin;

impl Plugin for JunctionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_lookup)
            .add_observer(zone_updated)
            .add_observer(switch_updated);
    }
}

#[derive(Resource, Default, DerefMut, Deref)]
pub struct SwitchJunctionLookup(HashMap<Entity, Entity>);

fn init_lookup(mut commands: Commands) {
    commands.insert_resource(SwitchJunctionLookup::default());
}

#[derive(Component, Debug)]
pub enum Junction {
    Split1_2 {
        signal: Entity,
    },
    Merge2_1 {
        signals: [Entity; 2],
        // switch: Entity,
        control: usize,
    },
}

fn zone_updated(update: On<ZoneUpdate>, mut commands: Commands, junctions: Query<&Junction>) {
    let ZoneUpdate {
        zone: e_junction,
        status,
    } = update.event();

    let Ok(junction) = junctions.get(*e_junction) else {
        return;
    };

    match junction {
        Junction::Merge2_1 { signals, control } => {
            signal_aspects(&mut commands, *status, *control, signals)
        }
        Junction::Split1_2 { signal } => {
            let aspect = match status {
                ZoneStatus::Clear => Aspect::Green,
                ZoneStatus::Occupied => Aspect::Red,
            };
            commands.trigger(SignalControl {
                signal: *signal,
                aspect,
            });
        }
    }
}

fn switch_updated(
    updated: On<SwitchUpdate>,
    mut commands: Commands,
    mut junctions: Query<(&mut Junction, &Zone)>,
    junction_lookup: Res<SwitchJunctionLookup>,
) {
    let SwitchUpdate {
        switch: e_switch,
        control: new_control,
    } = *updated.event();

    let Some(e_junction) = junction_lookup.get(&e_switch) else {
        return;
    };

    let Ok((mut junction, zone)) = junctions.get_mut(*e_junction) else {
        return;
    };

    match &mut *junction {
        Junction::Merge2_1 { signals, control } => {
            *control = new_control;
            signal_aspects(&mut commands, zone.status(), new_control, signals);
        }
        _ => return,
    }
}

fn signal_aspects(commands: &mut Commands, status: ZoneStatus, control: usize, signals: &[Entity]) {
    match status {
        ZoneStatus::Occupied => {
            commands.trigger(SignalControl {
                signal: signals[0],
                aspect: Aspect::Red,
            });
            commands.trigger(SignalControl {
                signal: signals[1],
                aspect: Aspect::Red,
            });
        }
        ZoneStatus::Clear => {
            commands.trigger(SignalControl {
                signal: signals[0],
                aspect: if control == 0 {
                    Aspect::Green
                } else {
                    Aspect::Red
                },
            });
            commands.trigger(SignalControl {
                signal: signals[1],
                aspect: if control == 1 {
                    Aspect::Green
                } else {
                    Aspect::Red
                },
            })
        }
    }
}
