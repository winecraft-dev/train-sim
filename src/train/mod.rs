use bevy::prelude::*;

pub mod axle;

use axle::{Axle, AxlePlugin};

use crate::{
    control::{ClickTarget, TargetClicked},
    switch::TrackSwitch,
    track::TrackSegment,
};

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AxlePlugin)
            .add_observer(train_derailed)
            .add_observer(train_clicked)
            .add_systems(Update, apply_train_speeds);
    }
}

#[derive(Event)]
pub struct TrainCreated {
    train: Entity,
    track: Entity,
}

#[derive(Event)]
pub struct TrainDerailed(Entity);

#[derive(Component)]
pub struct Derailed;

#[derive(Component, Default, Debug)]
pub struct Train {
    speed: f32,
}

impl Train {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn create(self, mut commands: Commands, track: Entity) -> Entity {
        let train = commands.spawn((ClickTarget, self)).id();
        commands.trigger(TrainCreated { train, track });
        train
    }
}

// fn apply_train_speeds(
//     mut commands: Commands,
//     trains: Query<(Entity, &Train, &mut Axle), Without<Derailed>>,
//     segments: Query<&TrackSegment>,
//     switches: Query<&TrackSwitch>,
// ) {
//     for (e_train, train, mut main_axle) in trains {
//         match main_axle.apply_speed(train.speed, segments, switches) {
//             Ok(_) => {}
//             Err(_) => {
//                 commands.trigger(TrainDerailed(e_train));
//                 return;
//             }
//         };
//         commands.trigger(TrainMoved(e_train));
//     }
// }

fn train_derailed(
    derailed: On<TrainDerailed>,
    mut commands: Commands,
    mut trains: Query<&mut Train>,
) {
    let e_train = derailed.0;
    let mut train = trains.get_mut(e_train).unwrap();

    train.speed = 0.0;
    commands.entity(e_train).insert(Derailed);
}

fn train_clicked(clicked: On<TargetClicked>, mut trains: Query<&mut Train>) {
    let e_train = clicked.0;
    let mut train = match trains.get_mut(e_train) {
        Ok(t) => t,
        Err(_) => return,
    };
    train.speed *= -1.0;
}
