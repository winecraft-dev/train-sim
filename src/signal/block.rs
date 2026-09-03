use bevy::prelude::*;

use crate::{
    loc::FacingLocation,
    signal::landmark::{Landmark, LandmarkPassed},
};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(check_train_passed)
            .add_observer(handle_train_entered)
            .add_observer(handle_train_exited);
    }
}

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct OccupiedBlock(Entity);

fn handle_train_entered(
    train_entered: On<TrainEnteredBlock>,
    mut commands: Commands,
    blocks: Query<Option<&OccupiedBlock>, With<Block>>,
) {
    let TrainEnteredBlock { block, train } = train_entered.event();

    let occupied = blocks.get(*block).unwrap();
    match occupied {
        Some(_) => {}
        None => {
            commands.entity(*block).insert(OccupiedBlock(*train));
            println!("Train[{}] entered block[{}]", train, block);
        }
    }
}

fn handle_train_exited(
    train_exited: On<TrainExitedBlock>,
    mut commands: Commands,
    blocks: Query<Option<&OccupiedBlock>, With<Block>>,
) {
    let TrainExitedBlock {
        block: e_block,
        train: e_train,
    } = train_exited.event();

    let occupied = blocks.get(*e_block).unwrap();
    match occupied {
        Some(_) => {
            commands.entity(*e_block).remove::<OccupiedBlock>();
        }
        None => {}
    }
}

#[derive(Component, Debug)]
pub struct BlockBound {
    block: Entity,
}

impl BlockBound {
    fn new(block: Entity) -> Self {
        Self { block }
    }
}

pub fn create_block(commands: &mut Commands, start: FacingLocation, end: FacingLocation) -> Entity {
    let e_block = commands.spawn((Block, Transform::default())).id();

    let e_start = commands
        .spawn((Landmark, BlockBound::new(e_block), start))
        .id();
    let e_end = commands
        .spawn((Landmark, BlockBound::new(e_block), end))
        .id();

    commands.entity(e_block).add_children(&[e_start, e_end]);
    e_block
}

#[derive(Event)]
pub struct TrainEnteredBlock {
    block: Entity,
    train: Entity,
}

#[derive(Event)]
pub struct TrainExitedBlock {
    block: Entity,
    train: Entity,
}

fn check_train_passed(
    passed: On<LandmarkPassed>,
    mut commands: Commands,
    bounds: Query<&BlockBound>,
) {
    let LandmarkPassed {
        forwards,
        landmark: e_landmark,
        train: e_train,
    } = *passed;

    let bound = match bounds.get(e_landmark) {
        Ok(b) => b,
        Err(_) => return,
    };

    match forwards {
        true => commands.trigger(TrainEnteredBlock {
            block: bound.block,
            train: e_train,
        }),
        false => commands.trigger(TrainExitedBlock {
            block: bound.block,
            train: e_train,
        }),
    };
}
