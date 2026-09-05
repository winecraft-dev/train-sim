use bevy::prelude::*;

use crate::{
    loc::FacingLocation,
    signal::landmark::{Landmark, LandmarkPassed},
};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(check_train_passed)
            .add_observer(train_passed);
    }
}

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct OccupiedBlock(Entity);

fn train_passed(
    passed: On<TrainPassedBlock>,
    mut commands: Commands,
    blocks: Query<Option<&OccupiedBlock>, With<Block>>,
) {
    let TrainPassedBlock {
        entered,
        block: e_block,
        train: e_train,
    } = *passed.event();

    let occupied = blocks.get(e_block).unwrap().is_some();
    if !occupied && entered {
        commands.entity(e_block).insert(OccupiedBlock(e_train));
    } else if occupied && !entered {
        commands.entity(e_block).remove::<OccupiedBlock>();
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
pub struct TrainPassedBlock {
    pub entered: bool,
    pub block: Entity,
    pub train: Entity,
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

    commands.trigger(TrainPassedBlock {
        entered: forwards,
        block: bound.block,
        train: e_train,
    });
}
