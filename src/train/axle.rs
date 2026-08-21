use bevy::prelude::*;

use crate::{
    switch::TrackSwitch,
    track::{TrackNode, TrackSegment, TrackVariant},
    train::{TrainDerailed, TrainMoved, cursor::TrackTraversal},
};

use super::TrainCreated;

pub struct AxlePlugin;

impl Plugin for AxlePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_axles)
            .add_observer(follow_main_axle)
            .add_systems(Update, project_axle_positions);
    }
}

#[derive(Component, Clone, Debug)]
pub struct Axle {
    pub track: Entity,
    pub distance: f32,
    pub traversal: TrackTraversal,
}

impl Axle {
    fn on_track(e_track: Entity, segments: Query<&TrackSegment>) -> Self {
        let track = segments.get(e_track).unwrap();
        let length = track.length();
        Self {
            track: e_track,
            distance: length / 2.0,
            traversal: TrackTraversal::FacingB,
        }
    }
}

#[derive(Component, Debug)]
pub struct AxleOffset(pub(super) f32);

const AXLE_DISTANCE: f32 = 50.0;

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

    let main_axle = Axle::on_track(*e_segment, segments);
    let rear_axle_offset = AxleOffset(AXLE_DISTANCE);

    let mut cursor = main_axle.clone();
    let rear_axle = match cursor.next_offset(&rear_axle_offset, segments, switches) {
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
        .spawn((rear_axle, rear_axle_offset, Transform::default()))
        .id();
    commands
        .entity(*e_train)
        .insert((main_axle, Transform::default()))
        .add_child(e_rear);
}

fn follow_main_axle(
    main_axle_moved: On<TrainMoved>,
    mut commands: Commands,
    children: Query<&Children>,
    main_axles: Query<&Axle, Without<AxleOffset>>,
    mut rear_axles: Query<(&mut Axle, &AxleOffset)>,
    segments: Query<&TrackSegment>,
    switches: Query<&TrackSwitch>,
) {
    let e_main = main_axle_moved.0;
    let e_rear = children.get(e_main).unwrap()[0];

    let main_axle = main_axles.get(e_main).unwrap();
    let (mut rear_axle, offset) = rear_axles.get_mut(e_rear).unwrap();

    let mut cursor = main_axle.clone();
    *rear_axle = match cursor.next_offset(offset, segments, switches) {
        Ok(a) => a,
        Err(_) => {
            commands.trigger(TrainDerailed(e_main));
            return;
        }
    };
}

fn project_axle_positions(
    axles: Query<(&mut Transform, &Axle)>,
    nodes: Query<&Transform, (With<TrackNode>, Without<Axle>)>,
    segments: Query<&TrackSegment>,
) {
    for (mut transform, axle) in axles {
        let segment = segments.get(axle.track).unwrap();
        let projected_position = project_axle_position(axle, segment, nodes);

        transform.translation = projected_position.extend(0.0);
    }
}

fn project_axle_position(
    axle: &Axle,
    segment: &TrackSegment,
    nodes: Query<&Transform, (With<TrackNode>, Without<Axle>)>,
) -> Vec2 {
    let a = nodes.get(segment.nodes.0).unwrap().translation.xy();
    let b = nodes.get(segment.nodes.1).unwrap().translation.xy();

    match segment.variant {
        TrackVariant::Straight => {
            let projected = a.lerp(b, axle.distance / segment.length());
            projected
        }
        TrackVariant::Curved {
            center,
            angle,
            radius,
        } => {
            let center = nodes.get(center).unwrap().translation.xy();
            let start_angle = (a - center).to_angle();

            let angle = angle.unwrap();
            let track_radius = radius.unwrap();

            let delta_angle = 0.0.lerp(angle, axle.distance / segment.length());
            let theta = start_angle + delta_angle;

            let (sin, cos) = ops::sin_cos(theta);
            let x = cos * track_radius;
            let y = sin * track_radius;
            let position = Vec2::new(x, y) + center;

            position
        }
    }
}
