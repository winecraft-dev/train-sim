use bevy::prelude::*;

use block::BlockPlugin;
use landmark::LandmarkPlugin;

use crate::{
    loc::FacingLocation,
    signal::{
        block::{Block, OccupiedBlock, TrainPassedBlock},
        landmark::{Landmark, LandmarkPassed},
    },
    train::effect::TrainEffect,
};

pub mod block;
pub mod error;
pub mod landmark;

pub struct SignalPlugin;

impl Plugin for SignalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LandmarkPlugin)
            .add_plugins(BlockPlugin)
            .add_observer(train_passed)
            .add_observer(train_exited);
    }
}

#[derive(Component)]
pub struct Signal {
    block: Entity,
}

#[derive(Component)]
pub struct HoldingSignal {
    train: Entity,
}

pub fn create_signal(commands: &mut Commands, block: Entity, location: FacingLocation) -> Entity {
    let e_signal = commands.spawn((Landmark, Signal { block }, location)).id();
    commands.entity(block).add_child(e_signal);
    e_signal
}

#[derive(Event)]
pub struct TrainSignaled {
    pub train: Entity,
    pub effect: TrainEffect,
}

fn train_passed(
    passed: On<LandmarkPassed>,
    mut commands: Commands,
    signals: Query<&Signal>,
    blocks: Query<Option<&OccupiedBlock>, With<Block>>,
) {
    let LandmarkPassed {
        forwards,
        landmark: e_landmark,
        train: e_train,
    } = *passed.event();

    if !forwards {
        return;
    }

    let signal = match signals.get(e_landmark) {
        Ok(s) => s,
        Err(_) => return,
    };

    let block_occupied = blocks.get(signal.block).unwrap().is_some();

    if block_occupied {
        commands
            .entity(e_landmark)
            .insert(HoldingSignal { train: e_train });
        commands.trigger(TrainSignaled {
            train: e_train,
            effect: TrainEffect::Stop,
        });
    }
}

fn train_exited(
    passed: On<TrainPassedBlock>,
    mut commands: Commands,
    signals: Query<(&HoldingSignal, &Signal)>,
    children: Query<&Children>,
) {
    let TrainPassedBlock {
        entered,
        block: e_block,
        train: _,
    } = *passed.event();

    if entered {
        return;
    }

    let signals: Vec<(&HoldingSignal, &Signal)> = children
        .get(e_block)
        .unwrap()
        .iter()
        .filter_map(|e| signals.get(e).ok())
        .collect();

    for (holding, _) in signals {
        let e_release = holding.train;

        commands.trigger(TrainSignaled {
            train: e_release,
            effect: TrainEffect::Go,
        });
    }
}
