use bevy::prelude::*;

use crate::track::TrackSegment;

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Debug, Component)]
pub struct Train {
    speed: f32,
    current_track: Option<Entity>,
}

impl Train {
    pub fn on_track(track: Entity) -> Self {
        Self {
            speed: 0.0,
            current_track: Some(track),
        }
    }
}
