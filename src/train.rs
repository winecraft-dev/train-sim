use bevy::prelude::*;

use crate::track::{TrackNode, TrackSegment, TrackVariant};

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_train_speeds, calculate_train_positions));
    }
}

#[derive(Debug, Component)]
pub struct Train {
    speed: f32,
    traversing: Option<Traversing>,
}

#[derive(Debug)]
struct Traversing {
    track: Entity,
    progress: f32,
}

impl Train {
    pub fn on_track(track: Entity) -> Self {
        Self {
            speed: 0.1,
            traversing: Some(Traversing {
                track,
                progress: 0.0,
            }),
        }
    }
}

// speed is a scalar value that does go negative... I guess that
// means that there's a direction to it. Since TrackSegments
// are given Nodes in arbitrary order, it is not feasible to maintain
// a positive/negative ordering from track to track. Therefore, the
// sign of the speed only applies to the current Traversing TrackSegment.
// Positive means node A->B, Negative means B->A
fn choose_direction(a: Vec2, b: Vec2, progress: f32) -> Vec2 {
    if progress >= 0.0 { a } else { b }
}

fn apply_train_speeds(trains: Query<&mut Train>) {
    for mut train in trains {
        let speed = train.speed;
        if let Some(traversing) = &mut train.traversing {
            traversing.progress += speed;
        }

        // determine if the train begins traversing onto the next track
    }
}

fn calculate_train_positions(
    trains: Query<(&mut Transform, &Train)>,
    nodes: Query<&TrackNode>,
    segments: Query<&TrackSegment>,
) {
    for (mut transform, train) in trains {
        if let Some(traversing) = &train.traversing {
            let progress = traversing.progress;
            let segment = segments.get(traversing.track).unwrap();

            let projected_position = project_train_position(nodes, segment, progress);

            transform.translation = projected_position.extend(0.0);
        }
    }
}

fn project_train_position(nodes: Query<&TrackNode>, segment: &TrackSegment, progress: f32) -> Vec2 {
    let a = nodes.get(segment.nodes.0).unwrap().position;
    let b = nodes.get(segment.nodes.1).unwrap().position;

    match segment.variant {
        TrackVariant::Straight => {
            let unit = (b - a).normalize();
            let start = choose_direction(a, b, progress);

            let projected = start + unit * progress;

            println!("{} {} {} {}", progress, unit, start, projected);

            projected
        }
        TrackVariant::Curved {
            center,
            angle: _,
            radius,
        } => todo!(),
    }
}
