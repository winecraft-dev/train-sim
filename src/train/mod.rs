use bevy::prelude::*;

pub mod axle;
pub mod builder;
pub mod mover;
pub mod store;

use crate::{
    error::SimError::{self, LandmarkScanFailed},
    landmark::{AxleIntrusion, Landmark, LandmarkPassed, scanner::Scanner},
    loc::{Dir, Loc},
    signal::control::{Effect, SignalCommand},
    train::{
        axle::{Axle, project_axle_positions},
        mover::TrainMover,
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
    car_distance: f32,
    axle_distance: f32,
}

impl Train {
    fn forwards(&self) -> bool {
        if self.speed < 0.0 { false } else { true }
    }
}

#[derive(Component, Default, Debug)]
pub struct Car;

#[derive(Event)]
pub struct TrainDerailed(Entity);

#[derive(Component)]
pub struct StoppedTrain(pub f32);

#[derive(Component)]
pub struct Derailed;

pub struct TrainMoved {
    front: (Loc, Loc),
    rear: (Loc, Loc),
}

impl TrainMoved {
    fn speed_direction(mut self, speed: f32) -> Self {
        let flipped = self.front.0.dir != self.front.1.dir;
        self.front.0.dir = if speed < 0.0 { Dir::A } else { Dir::B };
        self.front.1.dir = if !flipped {
            self.front.0.dir
        } else {
            self.front.0.dir.flip()
        };
        let flipped = self.rear.0.dir != self.rear.1.dir;
        self.rear.0.dir = if speed < 0.0 { Dir::A } else { Dir::B };
        self.rear.1.dir = if !flipped {
            self.rear.0.dir
        } else {
            self.rear.0.dir.flip()
        };
        self
    }
}

fn move_trains(
    mut commands: Commands,
    scanner: Scanner,
    mut mover: TrainMover,
    mut trains: Query<(Entity, &mut Loc, &Train), (Without<Derailed>, Without<Landmark>)>,
) {
    for (e_train, mut loc, train) in trains.iter_mut() {
        let mut movement = match mover.move_train((e_train, *loc, train)) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Problem moving train[{}], derailing: {:?}", e_train, e);
                commands.trigger(TrainDerailed(e_train));
                continue;
            }
        };
        *loc = movement.front.1;

        movement = movement.speed_direction(train.speed);

        if let Err(e) = train_passed(&scanner, &mut commands, e_train, train.forwards(), movement) {
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
    e_train: Entity,
    forwards: bool,
    movement: TrainMoved,
) -> Result<(), SimError> {
    match scanner.scan(movement.front.0, movement.front.1) {
        Ok(p) => {
            for (e_landmark, aligned) in p {
                commands.trigger(LandmarkPassed {
                    landmark: e_landmark,
                    train: e_train,
                    intrusion: AxleIntrusion {
                        forwards,
                        aligned,
                        axle: Axle::Front,
                    },
                });
            }
        }
        Err(e) => return Err(LandmarkScanFailed(Box::new(e))),
    };
    match scanner.scan(movement.rear.0, movement.rear.1) {
        Ok(p) => {
            for (e_landmark, aligned) in p {
                commands.trigger(LandmarkPassed {
                    landmark: e_landmark,
                    train: e_train,
                    intrusion: AxleIntrusion {
                        forwards,
                        aligned,
                        axle: Axle::Rear,
                    },
                });
            }
        }
        Err(e) => return Err(LandmarkScanFailed(Box::new(e))),
    };

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
