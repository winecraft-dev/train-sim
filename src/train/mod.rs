use bevy::prelude::*;

pub mod axle;
pub mod builder;
pub mod store;

use axle::AxlePlugin;

use crate::{
    signal::control::{Effect, SignalCommand},
    train::store::AxleStore,
};

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AxlePlugin)
            .add_systems(PreStartup, AxleStore::init)
            .add_observer(train_derailed)
            // .add_observer(train_clicked)
            .add_observer(train_signaled);
    }
}

#[derive(Component, Clone, Default, Debug)]
pub struct Train {
    speed: f32,
    car_distance: f32,
    axle_distance: f32,
}

#[derive(Component, Default, Debug)]
pub struct Car;

#[derive(Event)]
pub struct TrainDerailed(Entity);

#[derive(Component)]
pub struct StoppedTrain(pub f32);

#[derive(Component)]
pub struct Derailed;

fn train_derailed(
    derailed: On<TrainDerailed>,
    mut commands: Commands,
    mut trains: Query<&mut Train>,
) {
    let e_train = derailed.0;
    let mut train = match trains.get_mut(e_train) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Problem getting train[{}] to derail", e_train);
            return;
        }
    };

    train.speed = 0.0;
    commands.entity(e_train).insert(Derailed);
}

fn train_signaled(
    signaled: On<SignalCommand>,
    mut commands: Commands,
    mut trains: Query<(&mut Train, Option<&StoppedTrain>)>,
) {
    let SignalCommand {
        effect,
        train: e_train,
        signal: _,
    } = *signaled.event();

    let (mut train, stopped) = match trains.get_mut(e_train) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Problem getting train[{}] to signal", e_train);
            return;
        }
    };

    match effect {
        Effect::Stop => {
            let old_speed = train.speed;
            commands.entity(e_train).insert(StoppedTrain(old_speed));
            train.speed = 0.0;
        }
        Effect::Go => {
            let old_speed = match stopped {
                Some(s) => s.0,
                None => return,
            };
            commands.entity(e_train).remove::<StoppedTrain>();
            train.speed = old_speed;
        }
    }
}
