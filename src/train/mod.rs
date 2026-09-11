use bevy::prelude::*;

pub mod axle;
pub mod effect;

use axle::AxlePlugin;

use crate::{
    control::{ClickTarget, TargetClicked},
    loc::FacingLocation,
    signal::TrainSignaled,
    train::effect::{StoppedTrain, TrainEffect},
};

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AxlePlugin)
            .add_observer(train_derailed)
            .add_observer(train_clicked)
            .add_observer(train_signaled);
    }
}

#[derive(Event)]
pub struct TrainCreated {
    train: Entity,
    f_loc: FacingLocation,
}

#[derive(Component, Default, Debug)]
pub struct Train {
    speed: f32,
}

pub fn create_train(commands: &mut Commands, speed: f32, floc: FacingLocation) -> Entity {
    let train = commands
        .spawn((GlobalTransform::default(), ClickTarget, Train { speed }))
        .id();
    commands.trigger(TrainCreated { train, f_loc: floc });
    train
}

#[derive(Event)]
pub struct TrainDerailed(Entity);

#[derive(Component)]
pub struct Derailed;

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

fn train_signaled(
    signaled: On<TrainSignaled>,
    mut commands: Commands,
    mut trains: Query<(&mut Train, Option<&StoppedTrain>)>,
) {
    let TrainSignaled {
        train: e_train,
        effect,
    } = *signaled.event();

    let (mut train, stopped) = trains.get_mut(e_train).unwrap();
    match effect {
        TrainEffect::Stop => {
            let old_speed = train.speed;
            commands.entity(e_train).insert(StoppedTrain(old_speed));
            train.speed = 0.0;
        }
        TrainEffect::Go => {
            let old_speed = match stopped {
                Some(s) => s.0,
                None => return,
            };
            commands.entity(e_train).remove::<StoppedTrain>();
            train.speed = old_speed;
        }
    }
}
