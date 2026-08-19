use bevy::prelude::*;

pub mod axle;
mod cursor;

use axle::{Axle, AxlePlugin};

use crate::{switch::TrackSwitch, track::TrackSegment};

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AxlePlugin)
            .add_systems(Update, apply_train_speeds);
    }
}

#[derive(Event)]
pub struct TrainCreated {
    train: Entity,
    track: Entity,
}

#[derive(Event)]
pub struct TrainMoved(Entity);

#[derive(Component, Default, Debug)]
pub struct Train {
    speed: f32,
}

impl Train {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn create(self, mut commands: Commands, track: Entity) -> Entity {
        let train = commands.spawn(self).id();
        commands.trigger(TrainCreated { train, track });
        train
    }
}

fn apply_train_speeds(
    mut commands: Commands,
    trains: Query<(Entity, &Train, &mut Axle)>,
    segments: Query<&TrackSegment>,
    switches: Query<&TrackSwitch>,
) {
    for (e_train, train, mut main_axle) in trains {
        main_axle.apply_speed(train.speed, segments, switches);
        commands.trigger(TrainMoved(e_train));
    }
}
