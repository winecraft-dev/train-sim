use bevy::prelude::*;

use block::BlockPlugin;

pub mod block;
pub mod error;

pub struct SignalPlugin;

impl Plugin for SignalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlockPlugin);
    }
}
