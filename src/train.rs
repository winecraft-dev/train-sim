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
    // speed always refers to the speed in relation to the "front" of the train.
    // Right now, there is no front since the train is graphically a circle.
    speed: f32,
    traversing: Option<Traversing>,
}

// Progress ALWAYS refers to the distance from A along the route A->B.
// It will always be positive unless there is a track overrun
#[derive(Debug)]
struct Traversing {
    track: Entity, // TrackSegment
    progress: f32, // [0..1]
    direction: Direction,
}

impl Traversing {
    fn apply_speed(&mut self, segment: &TrackSegment, speed: f32) {
        let speed = match self.direction {
            Direction::BackwardBA => -speed,
            Direction::ForwardAB => speed,
        };

        let length = segment.length.unwrap();
        let mut distance = self.progress * length;
        distance += speed;

        self.progress = distance / length;
    }

    fn distance(&self, segment: &TrackSegment) -> f32 {
        let length = segment.length.unwrap();
        self.progress * length
    }

    fn overflow_distance(&self, segment: &TrackSegment) -> Option<f32> {
        let length = segment.length.unwrap();
        if self.progress < 0.0 {
            let overflow_progress = self.progress;
            return Some(overflow_progress * length);
        } else if self.progress > 1.0 {
            let overflow_progress = 1.0 - self.progress;
            return Some(overflow_progress * length);
        }
        None
    }
}

// Direction is the modifier applied to speed based on the way the "front"
// of the train enters the current TrackSegment. If the
#[derive(Debug)]
enum Direction {
    ForwardAB,
    BackwardBA,
}

impl Train {
    pub fn on_track(track: Entity) -> Self {
        Self {
            speed: 0.5,
            traversing: Some(Traversing {
                track,
                progress: 0.0,
                direction: Direction::ForwardAB,
            }),
        }
    }

    pub fn with_progress(mut self, progress: f32) -> Self {
        if progress < 0.0 || progress > 1.0 {
            return self;
        }
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

fn apply_train_speeds(
    trains: Query<&mut Train>,
    segments: Query<&TrackSegment>,
    nodes: Query<&TrackNode>,
) {
    for mut train in trains {
        let speed = train.speed;
        let traversing = match &mut train.traversing {
            None => continue,
            Some(traversing) => traversing,
        };

        let segment = segments.get(traversing.track).unwrap();
        traversing.apply_speed(segment, speed);

        let overflow_distance = match traversing.overflow_distance(segment) {
            Some(od) => od,
            None => continue,
        };
        let e_exit_node = if overflow_distance > 0.0 {
            match traversing.direction {
                Direction::ForwardAB => segment.nodes.1,
                Direction::BackwardBA => segment.nodes.0,
            }
        } else {
            match traversing.direction {
                Direction::ForwardAB => segment.nodes.0,
                Direction::BackwardBA => segment.nodes.1,
            }
        };

        let exit_node = nodes.get(e_exit_node).unwrap();
        let e_next_segment = match exit_node.next_track(traversing.track) {
            Some(segment) => segment,
            None => {
                train.speed = 0.0; // stop the train, we're done
                continue;
            }
        };

        traversing.track = e_next_segment;
        traversing.direction = Direction::ForwardAB;
        traversing.progress = 0.0;
    }
}

fn calculate_train_positions(
    trains: Query<(&mut Transform, &Train)>,
    nodes: Query<&TrackNode>,
    segments: Query<&TrackSegment>,
) {
    for (mut transform, train) in trains {
        if let Some(traversing) = &train.traversing {
            let segment = segments.get(traversing.track).unwrap();
            let projected_position = project_train_position(traversing, segment, nodes);

            transform.translation = projected_position.extend(0.0);
        }
    }
}

fn project_train_position(
    traversing: &Traversing,
    segment: &TrackSegment,
    nodes: Query<&TrackNode>,
) -> Vec2 {
    let a = nodes.get(segment.nodes.0).unwrap().position;
    let b = nodes.get(segment.nodes.1).unwrap().position;

    match segment.variant {
        TrackVariant::Straight => {
            let projected = a.lerp(b, traversing.progress);
            projected
        }
        TrackVariant::Curved {
            center,
            angle: _,
            radius,
        } => {
            let center = nodes.get(center).unwrap().position;
            let angle_a = (a - center).to_angle();
            let angle_b = (b - center).to_angle();

            let track_radius = radius.unwrap();

            let theta = angle_a.lerp(angle_b, traversing.progress);
            let (sin, cos) = ops::sin_cos(theta);
            let x = cos * track_radius;
            let y = sin * track_radius;
            let position = Vec2::new(x, y) + center;

            position
        }
    }
}
