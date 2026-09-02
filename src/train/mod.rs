use bevy::prelude::*;

pub mod axle;

use axle::AxlePlugin;

use crate::{
    control::{ClickTarget, TargetClicked},
    loc::{Direction, FacingLocation, Location},
};

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AxlePlugin)
            .add_observer(train_derailed)
            .add_observer(train_clicked);
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

impl Train {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn create(self, commands: &mut Commands, location: Location) -> Entity {
        let train = commands.spawn((ClickTarget, self)).id();
        commands.trigger(TrainCreated {
            train,
            f_loc: (location, Direction::default()),
        });
        train
    }
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
