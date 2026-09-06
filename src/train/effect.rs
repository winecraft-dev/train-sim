use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum TrainEffect {
    Stop,
    Go,
}

#[derive(Component)]
pub struct StoppedTrain(pub f32);
