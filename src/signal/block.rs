use bevy::prelude::*;

use crate::{
    loc::{Direction, FacingLocation, Location, cursor::TrackCursor},
    signal::error::SignalError,
    train::axle::AxleMoved,
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
        Some(t) => println!("Block[{}] already occupied by train[{}]!", block, t.0),
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
            println!("Train[{}] exited block[{}]", e_train, e_block);
        }
        None => println!(
            "Block[{}] wasn't occupied, but train[{}] exited!",
            e_block, e_train
        ),
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

pub struct BlockBuilder {
    start: FacingLocation,
    end: FacingLocation,
}

impl BlockBuilder {
    pub fn bounds(start: FacingLocation, end: FacingLocation) -> Self {
        Self { start, end }
    }

    pub fn create(self, commands: &mut Commands) -> Entity {
        let e_block = commands.spawn((Block, Transform::default())).id();

        let start_id = commands.spawn((BlockBound::new(e_block), self.start)).id();
        let end_id = commands.spawn((BlockBound::new(e_block), self.end)).id();

        commands.entity(e_block).add_children(&[start_id, end_id]);
        e_block
    }
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
    moved: On<AxleMoved>,
    mut commands: Commands,
    bounds: Query<(&BlockBound, &Location, &Direction)>,
    cursor: TrackCursor,
) {
    for (bound, bound_loc, bound_dir) in bounds {
        let AxleMoved {
            train: e_train,
            from,
            to,
        } = *moved;
        let passed = match cursor.passed(from, to, *bound_loc) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        let pass_dir = match passed {
            None => continue,
            Some(d) => d,
        };
        if pass_dir == *bound_dir {
            commands.trigger(TrainEnteredBlock {
                block: bound.block,
                train: e_train,
            })
        } else {
            commands.trigger(TrainExitedBlock {
                block: bound.block,
                train: e_train,
            });
        }
    }
}
