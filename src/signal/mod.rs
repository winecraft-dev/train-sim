use bevy::prelude::*;

use block::BlockPlugin;

use crate::{
    landmark::{Landmark, LandmarkPassed},
    loc::{Direction, FacingLocation, Location, cursor::TrackCursor},
    signal::block::{Block, OccupiedBlock, TrainPassedBlock},
    train::effect::TrainEffect,
};

pub mod block;
pub mod error;

pub struct SignalPlugin;

impl Plugin for SignalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlockPlugin)
            .add_observer(train_passed)
            .add_observer(train_exited)
            .add_systems(Update, add_observable_bound);
    }
}

#[derive(Component)]
pub struct Signal {
    pub block: Entity,
    stop_distance: f32,
}

#[derive(Component)]
pub struct HoldingSignal {
    train: Entity,
}

#[derive(Component)]
pub struct SignalWithBound;

#[derive(Component)]
pub struct ObservableBound {
    pub signal: Entity,
}

pub fn create_signal(
    commands: &mut Commands,
    block: Entity,
    stop_distance: f32,
    location: FacingLocation,
) -> Entity {
    let e_signal = commands
        .spawn((
            Signal {
                block,
                stop_distance,
            },
            location,
        ))
        .id();
    commands.entity(block).add_child(e_signal);
    e_signal
}

fn add_observable_bound(
    mut commands: Commands,
    cursor: TrackCursor,
    signals: Query<(Entity, &Location, &Direction, &Signal), Without<SignalWithBound>>,
) {
    for (e_signal, loc, dir, signal) in signals {
        let obv_loc = match cursor.traverse((*loc, *dir), signal.stop_distance) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Problem adding observable bound: {}", e);
                return;
            }
        };
        let e_obv = commands
            .spawn((Landmark, ObservableBound { signal: e_signal }, obv_loc))
            .id();
        commands
            .entity(e_signal)
            .add_child(e_obv)
            .insert(SignalWithBound);
    }
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
