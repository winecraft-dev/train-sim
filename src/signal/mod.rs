use bevy::prelude::*;

use crate::{
    landmark::LandmarkPassed, signal::builder::add_observable_bounds, train::effect::TrainEffect,
    zone::ZoneUpdate,
};

pub mod builder;
pub mod error;

pub struct SignalPlugin;

impl Plugin for SignalPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(train_passed)
            .add_observer(train_exited)
            .add_systems(PostStartup, add_observable_bounds);
    }
}

#[derive(Component)]
pub struct Signal {
    pub zone: Entity,
    pub observable_distance: f32,
}

#[derive(Component)]
pub struct HoldingSignal {
    train: Entity,
}

#[derive(Component)]
pub struct ObservableBound {
    pub signal: Entity,
}

#[derive(Event)]
pub struct TrainSignaled {
    pub train: Entity,
    pub effect: TrainEffect,
}

fn train_passed(
    passed: On<LandmarkPassed>,
    mut commands: Commands,
    obv_bounds: Query<&ObservableBound>,
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

    let e_signal = match obv_bounds.get(e_landmark) {
        Ok(s) => s.signal,
        Err(_) => return,
    };

    let signal = match signals.get(e_signal) {
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
    passed: On<ZoneUpdate>,
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
