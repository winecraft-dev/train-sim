use bevy::prelude::*;

use crate::{
    loc::{Location, cursor::TrackCursor},
    signal::error::SignalError,
    train::axle::AxleMoved,
};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(check_train_passed);
    }
}

#[derive(Component)]
pub struct Block;

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
    start: Location,
    end: Location,
}

impl BlockBuilder {
    pub fn bounds(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    pub fn create(self, mut commands: Commands) -> Result<(), SignalError> {
        let e_block = commands.spawn(Block).id();

        commands.spawn((BlockBound::new(e_block), self.start));
        commands.spawn((BlockBound::new(e_block), self.end));

        Ok(())
    }
}

fn check_train_passed(
    moved: On<AxleMoved>,
    bounds: Query<(&BlockBound, &Location)>,
    cursor: TrackCursor,
) {
    for (bound, bound_loc) in bounds {
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
        if passed {
            println!(
                "Train[{}] passed bound: {:?} {:?}",
                e_train, bound, bound_loc
            );
        }
    }
}
