use bevy::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Block {}

pub struct BlockBound {}
