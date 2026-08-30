use bevy::prelude::*;

use crate::{
    loc::{Direction, FacingLocation, Location, cursor::TrackCursor, projector::Projector},
    track::TrackNode,
    train::{Derailed, Train, TrainCreated, TrainDerailed},
};

pub struct AxlePlugin;

impl Plugin for AxlePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_axles)
            .add_systems(Update, (apply_train_speeds, project_axle_positions).chain());
    }
}

#[derive(Component, Clone, Debug)]
pub struct Axle {
    offset: f32,
}

impl Axle {
    fn new(offset: f32) -> Self {
        Self { offset }
    }
}

pub const AXLE_DISTANCE: f32 = 50.0;

fn axle_offset(facing: Direction, offset: f32) -> f32 {
    match facing {
        Direction::FacingA => offset,
        Direction::FacingB => -offset,
    }
}

fn add_axles(train_created: On<TrainCreated>, mut commands: Commands, cursor: TrackCursor) {
    let TrainCreated {
        train: e_train,
        f_loc,
    } = train_created.event();

    let main_floc = *f_loc;

    let main_axle = Axle::new(0.0);
    let rear_axle = Axle::new(AXLE_DISTANCE);

    let offset = axle_offset(main_floc.1, AXLE_DISTANCE);
    let rear_floc = match cursor.traverse(main_floc, offset) {
        Ok(a) => a,
        Err(e) => {
            eprintln!(
                "Skipping train[{}], problem with Track Cursor {:?}",
                e_train, e,
            );
            return;
        }
    };

    let e_rear = commands.spawn((rear_floc, rear_axle)).id();
    commands
        .entity(*e_train)
        .insert((main_floc, main_axle))
        .add_child(e_rear);
}

fn train_speed(facing: Direction, speed: f32) -> f32 {
    match facing {
        Direction::FacingA => -speed,
        Direction::FacingB => speed,
    }
}

fn movement_direction(speed: f32) -> Direction {
    if speed < 0.0 {
        Direction::FacingA
    } else {
        Direction::FacingB
    }
}

#[derive(Event)]
pub struct AxleMoved {
    pub train: Entity,
    pub from: FacingLocation,
    // facing direction refers to the direction the Axle moved, not just which direction it is facing
    pub to: FacingLocation,
}

fn apply_train_speeds(
    mut commands: Commands,
    trains: Query<(Entity, &Train, &Axle, &mut Location, &mut Direction), Without<Derailed>>,
    children: Query<&Children>,
    mut axles: Query<(&Axle, &mut Location, &mut Direction), Without<Train>>,
    cursor: TrackCursor,
) {
    for (e_train, train, _, mut main_loc, mut main_dir) in trains {
        let mut old_train_floc = (*main_loc, *main_dir);
        let speed = train_speed(old_train_floc.1, train.speed);
        let children = children.get(e_train).unwrap();

        let mut new_train_floc = match cursor.traverse(old_train_floc, speed) {
            Ok(l) => l,
            Err(_) => {
                commands.trigger(TrainDerailed(e_train));
                continue;
            }
        };

        *main_loc = new_train_floc.0;
        *main_dir = new_train_floc.1;

        let mut axle_floc = new_train_floc;
        for e_axle in children {
            let (next_axle, mut next_loc, mut next_dir) = axles.get_mut(*e_axle).unwrap();

            let offset = axle_offset(axle_floc.1, next_axle.offset);
            axle_floc = match cursor.traverse(axle_floc, offset) {
                Ok(loc) => loc,
                Err(_) => {
                    commands.trigger(TrainDerailed(e_train));
                    return;
                }
            };
            *next_loc = axle_floc.0;
            *next_dir = axle_floc.1;
        }

        // probably a bug here... maybe it will get exposed when a train
        // traverses beyond a single segment. Edge case that might turn
        // up later.
        old_train_floc.1 = movement_direction(speed);
        new_train_floc.1 = movement_direction(speed);

        commands.trigger(AxleMoved {
            train: e_train,
            from: old_train_floc,
            to: new_train_floc,
        });
    }
}

fn project_axle_positions(
    axles: Query<(Entity, &mut Transform, &Location), (With<Axle>, Without<TrackNode>)>,
    projector: Projector,
) {
    for (e_axle, mut transform, location) in axles {
        transform.translation = match projector.project(*location) {
            Ok(pos) => pos,
            Err(e) => {
                eprintln!("Problem with axle[{}], skipping: {}", e_axle, e);
                continue;
            }
        };
    }
}
