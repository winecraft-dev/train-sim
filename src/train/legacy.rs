use bevy::prelude::*;

mod axle;

use crate::{
    control::TargetClicked,
    switch::TrackSwitch,
    track::{TrackNode, TrackSegment, TrackVariant},
};

use axle::AxelPlugin;

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AxelPlugin)
            .add_systems(
                Update,
                (
                    apply_train_speeds,
                    switch_overflow_trains,
                    calculate_train_positions,
                )
                    .chain(),
            )
            .add_observer(train_clicked);
    }
}

// TrackTraversal is the modifier applied to speed based on the way the "front"
// of the train enters the current TrackSegment.
#[derive(Debug)]
pub enum TrackTraversal {
    FacingA,
    FacingB,
}

#[derive(Debug, Component)]
pub struct Train {
    // speed always refers to the speed in relation to the "front" of the train.
    // Right now, there is no front since the train is graphically a circle.
    speed: f32,
    track: Entity,
    progress: f32,
    direction: TrackTraversal,
}

impl Train {
    pub fn on_track(track: Entity, direction: TrackTraversal) -> Self {
        let progress = match direction {
            TrackTraversal::FacingA => 0.0,
            TrackTraversal::FacingB => 1.0,
        };
        Self {
            speed: 0.0,
            track,
            progress,
            direction,
        }
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    // Progress ALWAYS refers to the distance from A along the route A->B.
    // It will always be positive unless there is a track overrun
    fn apply_speed(&mut self, segment: &TrackSegment, speed: f32) {
        print!("speed: {:.2} {:?}", speed, self.direction);
        let speed = match self.direction {
            Direction::Backward => -speed,
            Direction::Forward => speed,
        };
        print!("with direction: {:.2}", speed);

        let length = segment.length();
        let mut distance = self.progress * length;
        distance += speed;

        self.progress = distance / length;
    }

    fn exited(&self, segment: &TrackSegment) -> Option<(Entity, f32)> {
        let length = segment.length();
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

    fn traverse_next(
        &mut self,
        next_segment: Entity,
        origin_node: Entity,
        segments: Query<&TrackSegment>,
        overflow_distance: f32,
    ) {
        self.track = next_segment;

        let next_segment = segments.get(next_segment).unwrap();
        let overflow_progress = overflow_distance / next_segment.length();
        if next_segment.nodes.0 == origin_node {
            // starting on A side
            self.direction = if self.speed >= 0.0 {
                Direction::Forward // moving forwards, towards B
            } else {
                Direction::Backward // moving backwards, toward B
            };
            self.progress = overflow_progress;
        } else {
            // starting on B side
            self.direction = if self.speed >= 0.0 {
                Direction::Backward // moving backwards, toward A
            } else {
                Direction::Forward // moving forwards, toward A
            };
            self.progress = 1.0 + overflow_progress;
        }
    }
}

fn apply_train_speeds(trains: Query<&mut Train>, segments: Query<&TrackSegment>) {
    for mut train in trains {
        let speed = train.speed;

        let segment = segments.get(train.track).unwrap();
        train.apply_speed(segment, speed);
    }
}

fn switch_overflow_trains(
    trains: Query<&mut Train>,
    segments: Query<&TrackSegment>,
    switches: Query<&TrackSwitch>,
) {
    for mut train in trains {
        let e_current = train.track;
        let current = segments.get(e_current).unwrap();
        let (e_switch, overflow_distance) = match train.exited(current) {
            Some(exit) => exit,
            None => continue,
        };

        let switch = switches.get(e_switch).unwrap();
        let e_next = match switch.next_segment(e_current) {
            Some(segment) => segment,
            None => {
                train.trim_progress();
                train.set_speed(0.0);
                continue;
            }
        };

        train.traverse_next(e_next, e_switch, segments, overflow_distance);
    }
}

fn calculate_train_positions(
    trains: Query<(&mut Transform, &Train)>,
    nodes: Query<&Transform, (With<TrackNode>, Without<Train>)>,
    segments: Query<&TrackSegment>,
) {
    for (mut transform, train) in trains {
        let segment = segments.get(train.track).unwrap();
        let projected_position = project_train_position(train, segment, nodes);

        transform.translation = projected_position.extend(0.0);
    }
}

fn project_train_position(
    train: &Train,
    segment: &TrackSegment,
    nodes: Query<&Transform, (With<TrackNode>, Without<Train>)>,
) -> Vec2 {
    let a = nodes.get(segment.nodes.0).unwrap().translation.xy();
    let b = nodes.get(segment.nodes.1).unwrap().translation.xy();
    println!("progress: {:.2}", train.progress);

    match segment.variant {
        TrackVariant::Straight => {
            let projected = a.lerp(b, train.progress);
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

            let delta_angle = 0.0.lerp(angle, train.progress);
            let theta = start_angle + delta_angle;

            let (sin, cos) = ops::sin_cos(theta);
            let x = cos * track_radius;
            let y = sin * track_radius;
            let position = Vec2::new(x, y) + center;

            position
        }
    }
}

fn train_clicked(clicked: On<TargetClicked>, mut trains: Query<&mut Train>) {
    let e_train = clicked.event().0;
    if let Ok(mut train) = trains.get_mut(e_train) {
        train.speed *= -1.0;
    }
}
