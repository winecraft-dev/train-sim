mod control;
mod error;
mod landmark;
mod loader;
mod loc;
mod render;
mod signal;
mod track;
mod train;
mod zone;

use bevy::prelude::*;
use rand::RngExt;

use crate::{
    control::ControlPlugin,
    landmark::LandmarkPlugin,
    loader::LoaderPlugin,
    loc::{Dir, LocationPlugin, locator::Locator},
    render::debug::DebugRenderPlugin,
    signal::SignalPlugin,
    track::{SwitchesSpawned, TrackPlugin},
    train::{TrainPlugin, create_train},
    zone::AxleCounterPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TrackPlugin)
        .add_plugins(LocationPlugin)
        .add_plugins(TrainPlugin)
        .add_plugins(LandmarkPlugin)
        .add_plugins(ControlPlugin)
        .add_plugins(SignalPlugin)
        .add_plugins(LoaderPlugin)
        .add_plugins(DebugRenderPlugin)
        .add_plugins(AxleCounterPlugin)
        .add_systems(Startup, config)
        // .add_observer(setup_blocks)
        .add_observer(setup_trains)
        .run();
}

fn config(mut config: ResMut<GizmoConfigStore>, mut commands: Commands) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;

    commands.spawn((Camera2d, Camera::default()));
}

fn setup_trains(_done: On<SwitchesSpawned>, mut commands: Commands, locator: Locator) {
    let locs = [
        locator.on_progress(0, 0.5, Dir::B),
        locator.on_progress(1, 0.5, Dir::B),
        locator.on_progress(2, 0.5, Dir::B),
        locator.on_progress(3, 0.5, Dir::B),
        locator.on_progress(4, 0.5, Dir::B),
        locator.on_progress(5, 0.5, Dir::B),
        locator.on_progress(6, 0.5, Dir::B),
        locator.on_progress(7, 0.5, Dir::B),
        locator.on_progress(8, 0.5, Dir::B),
    ];

    for loc in locs {
        let mut rng = rand::rng();
        create_train(&mut commands, rng.random_range(1.0..2.0), loc.unwrap());
    }
}
