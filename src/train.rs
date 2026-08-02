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

    fn exited(&self, segment: &TrackSegment) -> Option<(Entity, f32)> {
        let length = segment.length.unwrap();
        if self.progress < 0.0 {
            let overflow_progress = self.progress;
            let node_a = segment.nodes.0;
            return Some((node_a, overflow_progress * length));
        } else if self.progress > 1.0 {
            let overflow_progress = self.progress - 1.0;
            let node_b = segment.nodes.1;
            return Some((node_b, overflow_progress * length));
        }
        None
    }

    fn trim_progress(&mut self) {
        if self.progress < 0.0 {
            self.progress = 0.0;
        } else if self.progress > 1.0 {
            self.progress = 1.0;
        }
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
    pub fn forward(track: Entity) -> Self {
        Self {
            speed: 0.0,
            traversing: Some(Traversing {
                track,
                progress: 0.0,
                direction: Direction::ForwardAB,
            }),
        }
    }

    pub fn backward(track: Entity) -> Self {
        Self {
            speed: 0.0,
            traversing: Some(Traversing {
                track,
                progress: 1.0,
                direction: Direction::BackwardBA,
            }),
        }
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
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

        let (e_exit_node, overflow_distance) = match traversing.exited(segment) {
            Some(exit) => exit,
            None => continue,
        };

        println!(
            "od: {} segment: {:?} exit_node[{}]",
            overflow_distance, segment, e_exit_node
        );

        let exit_node = nodes.get(e_exit_node).unwrap();
        println!("exit_node[{}]: {:?}", e_exit_node, exit_node);
        let e_next_segment = match exit_node.next_track(traversing.track) {
            Some(segment) => segment,
            None => {
                traversing.trim_progress();
                train.set_speed(0.0);
                continue;
            }
        };

        println!(
            "exit_node[{}]: {:?}, next_segment[{}]",
            e_exit_node, exit_node, e_next_segment
        );

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
