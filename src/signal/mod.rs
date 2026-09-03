use bevy::prelude::*;

use block::BlockPlugin;
use landmark::LandmarkPlugin;

use crate::{
    loc::FacingLocation,
    signal::{
        block::{TrainEnteredBlock, TrainExitedBlock},
        landmark::{Landmark, LandmarkPassed},
    },
};

pub mod block;
pub mod error;
pub mod landmark;

pub struct SignalPlugin;

impl Plugin for SignalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LandmarkPlugin).add_plugins(BlockPlugin);
    }
}

#[derive(Component)]
pub struct Signal {
    block: Entity,
}

#[derive(Component)]
pub struct StopSignal;

pub fn create_signal(commands: &mut Commands, block: Entity, location: FacingLocation) -> Entity {
    commands.spawn((Landmark, Signal { block }, location)).id()
}

fn entered_block(entered: On<TrainEnteredBlock>) {}

fn exited_block(exited: On<TrainExitedBlock>) {}

fn handle_train_passed(passed: On<LandmarkPassed>, signals: Query<&Signal>) {}
