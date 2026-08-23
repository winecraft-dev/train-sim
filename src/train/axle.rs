use bevy::prelude::*;

use crate::{
    switch::TrackSwitch,
    track::{TrackNode, TrackSegment, cursor::TrackCursor, loc::Location, projector::Projector},
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

fn add_axles(
    train_created: On<TrainCreated>,
    mut commands: Commands,
    segments: Query<&TrackSegment>,
    switches: Query<&TrackSwitch>,
) {
    let TrainCreated {
        train: e_train,
        track: e_segment,
    } = train_created.event();

    let main_location = Location::on_track(*e_segment, segments).unwrap();
    let main_axle = Axle::new(0.0);

    let rear_axle = Axle::new(AXLE_DISTANCE);

    let mut cursor: TrackCursor = main_location.clone();
    let rear_location = match cursor.next_offset(AXLE_DISTANCE, segments, switches) {
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
        .insert((main_location, main_axle, Transform::default()))
        .add_child(e_rear);
}

fn apply_train_speeds(
    mut commands: Commands,
    trains: Query<(Entity, &Train, &Axle, &mut Location), Without<Derailed>>,
    children: Query<&Children>,
    mut axles: Query<(&Axle, &mut Location), Without<Train>>,
    segments: Query<&TrackSegment>,
    switches: Query<&TrackSwitch>,
) {
    for (e_train, train, _, mut main_loc) in trains {
        let speed = train.speed;
        let children = children.get(e_train).unwrap();

        match main_loc.apply_speed(speed, segments, switches) {
            Ok(_) => {}
            Err(_) => {
                commands.trigger(TrainDerailed(e_train));
                continue;
            }
        }

        let mut cursor = main_loc.clone();
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
