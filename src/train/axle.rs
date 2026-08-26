use bevy::prelude::*;

use crate::{
    track::{
        TrackNode, TrackSegment,
        cursor::TrackCursor,
        loc::{Direction, Location},
        projector::Projector,
    },
    train::{Derailed, Train, TrainDerailed},
};

use super::TrainCreated;

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

    let rear_location =
        match cursor.traverse_distance(*main_loc, axle_offset(*main_loc, AXLE_DISTANCE)) {
            Ok(a) => a,
            Err(e) => {
                eprintln!(
                    "Skipping train[{}], problem with Track Cursor {:?}",
                    e_train, e,
                );
                return;
            }
        };

    let e_rear = commands
        .spawn((rear_location, rear_axle, Transform::default()))
        .id();
    commands
        .entity(*e_train)
        .insert((*main_loc, main_axle, Transform::default()))
        .add_child(e_rear);
}

fn train_speed(loc: Location, speed: f32) -> f32 {
    match loc.direction {
        Direction::FacingA => -speed,
        Direction::FacingB => speed,
    }
}

fn apply_train_speeds(
    mut commands: Commands,
    trains: Query<(Entity, &Train, &Axle, &mut Location), Without<Derailed>>,
    children: Query<&Children>,
    mut axles: Query<(&Axle, &mut Location), Without<Train>>,
    cursor: TrackCursor,
) {
    for (e_train, train, _, mut main_loc) in trains {
        let speed = train.speed;
        let children = children.get(e_train).unwrap();

        *main_loc = match cursor.traverse_distance(*main_loc, train_speed(*main_loc, speed)) {
            Ok(l) => l,
            Err(_) => {
                commands.trigger(TrainDerailed(e_train));
                continue;
            }
        };

        let axle_loc = main_loc.clone();
        for e_axle in children {
            let (next_axle, mut next_loc) = axles.get_mut(*e_axle).unwrap();

            *next_loc = match cursor.next_offset(next_axle.offset, segments, switches) {
                Ok(loc) => loc,
                Err(_) => {
                    commands.trigger(TrainDerailed(e_train));
                    return;
                }
            };
        }
    }
}

fn project_axle_positions(
    axles: Query<(Entity, &mut Transform, &Location), (With<Axle>, Without<TrackNode>)>,
    segments: Query<&TrackSegment>,
    nodes: Query<&Transform, With<TrackNode>>,
) {
    for (e_axle, mut transform, location) in axles {
        let projector: Projector = *location;

        transform.translation = match projector.project(segments, nodes) {
            Ok(projected) => projected.extend(0.0),
            Err(e) => {
                eprintln!("Problem with axle[{}], skipping: {}", e_axle, e);
                continue;
            }
        };
    }
}
