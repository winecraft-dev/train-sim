mod control;
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
    zone::{
        AxleCounterPlugin,
        builder::{ZoneBuilder, ZoneConstructor},
    },
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
        // .add_observer(setup_trains)
        .run();
}

fn config(mut config: ResMut<GizmoConfigStore>, mut commands: Commands) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;

    commands.spawn((Camera2d, Camera::default()));
}

fn setup_trains(_done: On<SwitchesSpawned>, mut commands: Commands, locator: Locator) {
    let locs = [
        locator.on_progress(0, 0.5, Dir::ToB),
        locator.on_progress(1, 0.5, Dir::ToB),
        locator.on_progress(2, 0.5, Dir::ToB),
        locator.on_progress(3, 0.5, Dir::ToB),
        locator.on_progress(4, 0.5, Dir::ToB),
        locator.on_progress(5, 0.5, Dir::ToB),
        locator.on_progress(6, 0.5, Dir::ToB),
        locator.on_progress(7, 0.5, Dir::ToB),
        locator.on_progress(8, 0.5, Dir::ToB),
    ];

    for loc in locs {
        let mut rng = rand::rng();
        create_train(&mut commands, rng.random_range(1.0..2.0), loc.unwrap());
    }
}

fn setup_blocks(_done: On<SwitchesSpawned>, mut builder: ZoneBuilder, locator: Locator) {
    ZoneConstructor::new()
        .with_entry(locator.on_end(0, Dir::ToB, 10.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(0, Dir::ToB, 200.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(0, Dir::ToB, 210.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(0, Dir::ToB, 400.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(0, Dir::ToB, 410.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(0, Dir::ToA, 10.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(1, Dir::ToB, 0.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(1, Dir::ToA, 0.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(2, Dir::ToB, 0.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(2, Dir::ToB, 200.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(2, Dir::ToA, 20.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(11, Dir::ToB, 0.0, Dir::ToA).unwrap())
        .with_exit(locator.on_end(4, Dir::ToB, 100.0, Dir::ToA).unwrap())
        .with_switch(4)
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(11, Dir::ToB, 10.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(11, Dir::ToB, 200.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(11, Dir::ToB, 210.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(11, Dir::ToB, 400.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(11, Dir::ToB, 410.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(11, Dir::ToA, 10.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_exit(locator.on_end(9, Dir::ToA, 40.0, Dir::ToB).unwrap())
        .with_entry(locator.on_end(8, Dir::ToA, 100.0, Dir::ToB).unwrap())
        .with_entry(locator.on_end(11, Dir::ToA, 0.0, Dir::ToB).unwrap())
        .with_switch(15)
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(9, Dir::ToA, 60.0, Dir::ToA).unwrap())
        .with_exit(locator.on_end(9, Dir::ToB, 0.0, Dir::ToB).unwrap())
        .with_signal_distance(15.0)
        .with_observable_distance(20.0)
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(12, Dir::ToB, 0.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(12, Dir::ToA, 0.0, Dir::ToA).unwrap())
        .with_observable_distance(20.0)
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(4, Dir::ToB, 100.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(4, Dir::ToA, 0.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(5, Dir::ToB, 0.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(5, Dir::ToA, 0.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(6, Dir::ToB, 10.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(6, Dir::ToB, 200.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(6, Dir::ToB, 210.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(6, Dir::ToB, 400.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(6, Dir::ToB, 410.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(6, Dir::ToA, 10.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(7, Dir::ToB, 0.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(7, Dir::ToA, 0.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    ZoneConstructor::new()
        .with_entry(locator.on_end(8, Dir::ToB, 0.0, Dir::ToB).unwrap())
        .with_exit(locator.on_end(8, Dir::ToA, 110.0, Dir::ToA).unwrap())
        .build(&mut builder)
        .unwrap();

    builder.done();
}
