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

// Progress ALWAYS refers to the distance from A along the route A->B.
// It will always be positive unless there is a track overrun
#[derive(Debug)]
struct Traversing {
    track: Entity, // TrackSegment
    progress: f32,
}

impl Train {
    pub fn on_track(track: Entity) -> Self {
        Self {
            speed: 0.5,
            traversing: Some(Traversing {
                track,
                progress: 0.0,
            }),
        }
    }

    pub fn with_progress(mut self, progress: f32) -> Self {
        if let Some(traversing) = self.traversing.as_mut() {
            traversing.progress = progress;
        }
        self
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
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

            let projected = a + unit * progress;
            projected
        }
        TrackVariant::Curved {
            center,
            angle: _,
            radius,
        } => {
            let center = nodes.get(center).unwrap().position;
            let angle_a = (a - center).to_angle();

            let track_radius = radius.unwrap();
            let track_length = segment.length.unwrap();

            let theta = angle_a + progress / track_length;
            let (sin, cos) = ops::sin_cos(theta);
            let x = cos * track_radius;
            let y = sin * track_radius;
            let position = Vec2::new(x, y) + center;

            position
        }
    }
}
