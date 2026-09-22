use bevy::prelude::*;

use crate::{
    signal::{Aspect, control::SignalControl},
    zone::{ZoneStatus, ZoneUpdate},
};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(zone_updated);
    }
}

#[derive(Component)]
pub struct Block {
    pub entry_signal: Entity,
}

pub fn zone_updated(update: On<ZoneUpdate>, mut commands: Commands, blocks: Query<&Block>) {
    let ZoneUpdate {
        zone: e_block,
        status,
    } = update.event();

    let block = match blocks.get(*e_block) {
        Ok(b) => b,
        Err(_) => {
            return;
        }
    };

    let e_signal = block.entry_signal;
    match status {
        ZoneStatus::Clear => commands.trigger(SignalControl {
            signal: e_signal,
            aspect: Aspect::Green,
        }),
        ZoneStatus::Occupied => commands.trigger(SignalControl {
            signal: e_signal,
            aspect: Aspect::Red,
        }),
    };
}
