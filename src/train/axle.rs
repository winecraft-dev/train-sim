use bevy::prelude::*;

use crate::{
    loc::{Direction, Location, cursor::TrackCursor, projector::Projector},
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

fn axle_offset(loc: Location, offset: f32) -> f32 {
    match loc.direction {
        Direction::FacingA => offset,
        Direction::FacingB => -offset,
    }
}

fn add_axles(train_created: On<TrainCreated>, mut commands: Commands, cursor: TrackCursor) {
    let TrainCreated {
        train: e_train,
        location: main_loc,
    } = train_created.event();

    let main_axle = Axle::new(0.0);
    let rear_axle = Axle::new(AXLE_DISTANCE);

    let offset = axle_offset(*main_loc, AXLE_DISTANCE);
    let rear_location = match cursor.traverse(*main_loc, offset) {
        Ok(a) => a,
        Err(e) => {
            eprintln!(
                "Skipping train[{}], problem with Track Cursor {:?}",
                e_train, e,
            );
            return;
        }
    };

    let e_rear = commands.spawn((rear_location, rear_axle)).id();
    commands
        .entity(*e_train)
        .insert((*main_loc, main_axle))
        .add_child(e_rear);
}

fn train_speed(loc: Location, speed: f32) -> f32 {
    match loc.direction {
        Direction::FacingA => -speed,
        Direction::FacingB => speed,
    }
}

#[derive(Event)]
pub struct AxleMoved {
    pub train: Entity,
    pub from: Location,
    pub to: Location,
}

fn apply_train_speeds(
    mut commands: Commands,
    trains: Query<(Entity, &Train, &Axle, &mut Location), Without<Derailed>>,
    children: Query<&Children>,
    mut axles: Query<(&Axle, &mut Location), Without<Train>>,
    cursor: TrackCursor,
) {
    for (e_train, train, _, mut main_loc) in trains {
        let old_train_loc = *main_loc;
        let speed = train_speed(old_train_loc, train.speed);
        let children = children.get(e_train).unwrap();

        let new_train_loc = match cursor.traverse(old_train_loc, speed) {
            Ok(l) => l,
            Err(_) => {
                commands.trigger(TrainDerailed(e_train));
                continue;
            }
        };

        *main_loc = new_train_loc;

        let mut axle_loc = new_train_loc;
        for e_axle in children {
            let (next_axle, mut next_loc) = axles.get_mut(*e_axle).unwrap();

            let offset = axle_offset(axle_loc, next_axle.offset);
            axle_loc = match cursor.traverse(axle_loc, offset) {
                Ok(loc) => loc,
                Err(_) => {
                    commands.trigger(TrainDerailed(e_train));
                    return;
                }
            };
            *next_loc = axle_loc;
        }

        commands.trigger(AxleMoved {
            train: e_train,
            from: old_train_loc,
            to: new_train_loc,
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
