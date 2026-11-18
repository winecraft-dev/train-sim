use bevy::prelude::*;

pub mod axle;
pub mod builder;
pub mod mover;
pub mod store;

use crate::{
    error::SimError::{self, LandmarkScanFailed},
    landmark::{Landmark, LandmarkPassed, scanner::Scanner},
    loc::Loc,
    signal::control::{Effect, SignalCommand},
    train::{
        axle::project_axle_positions,
        mover::{TrainMoved, TrainMover},
        store::TrainStore,
    },
};

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, TrainStore::init)
            .add_systems(Update, (move_trains, project_axle_positions).chain())
            .add_observer(train_derailed)
            .add_observer(train_signaled);
    }
}

#[derive(Component, Clone, Default, Debug)]
pub struct Train {
    speed: f32,
    cars: usize,
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

fn move_trains(
    mut commands: Commands,
    scanner: Scanner,
    mut mover: TrainMover,
    mut trains: Query<(Entity, &mut Loc, &Train), (Without<Derailed>, Without<Landmark>)>,
) {
    for (e_train, mut loc, train) in trains.iter_mut() {
        let calculated = match mover.calculate_movement((*loc, train)) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Problem calc train[{}] move: {:?}", e_train, e);
                commands.trigger(TrainDerailed(e_train));
                continue;
            }
        };
        let mut moved = match mover.move_train((e_train, &mut loc, train), calculated) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Problem moving train[{}], derailing: {:?}", e_train, e);
                commands.trigger(TrainDerailed(e_train));
                continue;
            }
        };
        moved.speed_direction(train.speed);

        for axle in moved.movement.iter() {
            print!("{:?},", axle.from.dir);
        }
        println!("");

        if let Err(e) = train_passed(&scanner, &mut commands, moved) {
            eprintln!(
                "Problem checking if train[{}] passed landmarks: {:?}",
                e_train, e
            );
            continue;
        }
    }
}

fn train_passed(
    scanner: &Scanner,
    commands: &mut Commands,
    moved: TrainMoved,
) -> Result<(), SimError> {
    for axle_moved in moved.movement {
        match scanner.scan(axle_moved.from, axle_moved.to) {
            Ok(p) => {
                for (e_landmark, aligned) in p {
                    commands.trigger(LandmarkPassed {
                        landmark: e_landmark,
                        train: moved.train,
                        axle: axle_moved.axle,
                        aligned,
                    });
                }
            }
            Err(e) => return Err(LandmarkScanFailed(Box::new(e))),
        }
    }

    Ok(())
}

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
