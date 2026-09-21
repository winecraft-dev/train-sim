mod control;
mod landmark;
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
    loc::{
        Dir::{self, FacingA},
        LocationPlugin,
        locator::Locator,
    },
    render::debug::DebugRenderPlugin,
    signal::{SignalPlugin, builder::SignalBuilder},
    track::{
        SwitchesSpawned, TrackPlugin,
        builder::{TrackBuilder, TrackStore},
    },
    train::{TrainPlugin, create_train},
    zone::{
        AxleCounterPlugin,
        builder::{ZoneBuilder, ZoneStore, ZonesBuilt},
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
        .add_plugins(DebugRenderPlugin)
        .add_plugins(AxleCounterPlugin)
        .add_systems(Startup, (config, setup_nodes, setup_tracks).chain())
        .add_observer(setup_blocks)
        .add_observer(setup_trains)
        .add_observer(setup_signals)
        .run();
}

fn config(mut config: ResMut<GizmoConfigStore>, mut commands: Commands) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;

    commands.spawn((Camera2d, Camera::default()));
    commands.insert_resource(TrackStore::default());
    commands.insert_resource(ZoneStore::default());
}

fn setup_nodes(mut builder: TrackBuilder) {
    builder.node(-300.0, 300.0);
    builder.node(300.0, 300.0);
    builder.node(300.0, 250.0); // CENTER
    builder.node(350.0, 250.0);
    builder.node(350.0, 0.0);
    builder.node(300.0, 0.0); // CENTER
    builder.node(300.0, -50.0);
    builder.node(350.0, -250.0);
    builder.node(300.0, -250.0); // CENTER
    builder.node(300.0, -300.0);
    builder.node(-300.0, -300.0);
    builder.node(-300.0, -250.0); // CENTER
    builder.node(-350.0, -250.0);
    builder.node(-300.0, -50.0);
    builder.node(-300.0, 0.0); // CENTER
    builder.node(-350.0, 0.0);
    builder.node(-300.0, 250.0); // CENTER
    builder.node(-350.0, 250.0);
}

fn setup_tracks(mut builder: TrackBuilder) {
    builder.straight(0, 1);
    builder.curved(1, 3, 2);
    builder.straight(3, 4);
    builder.curved(4, 6, 5);
    builder.straight(4, 7);
    builder.curved(7, 9, 8);
    builder.straight(9, 10);
    builder.curved(10, 12, 11);
    builder.straight(12, 15);
    builder.straight(17, 15);
    builder.curved(13, 15, 14);
    builder.straight(6, 13);
    builder.curved(17, 0, 16);

    builder.flush();
}

fn setup_trains(_done: On<SwitchesSpawned>, mut commands: Commands, locator: Locator) {
    let locs = [
        locator.on_progress(0, 0.5, Dir::FacingB),
        locator.on_progress(1, 0.5, Dir::FacingB),
        locator.on_progress(2, 0.5, Dir::FacingB),
        locator.on_progress(3, 0.5, Dir::FacingB),
        locator.on_progress(4, 0.5, Dir::FacingB),
        locator.on_progress(5, 0.5, Dir::FacingB),
        locator.on_progress(6, 0.5, Dir::FacingB),
        locator.on_progress(7, 0.5, Dir::FacingB),
        locator.on_progress(8, 0.5, Dir::FacingB),
    ];

    for loc in locs {
        let mut rng = rand::rng();
        create_train(&mut commands, rng.random_range(1.0..3.0), loc);
    }
}

fn setup_blocks(_done: On<SwitchesSpawned>, mut builder: ZoneBuilder, locator: Locator) {
    let ez0 = builder.new();
    builder.add_counter(ez0, locator.on_end(0, Dir::FacingA, 10.0, Dir::FacingA));
    builder.add_counter(ez0, locator.on_end(0, Dir::FacingB, 10.0, Dir::FacingB));
    builder.block(ez0);

    let ez1 = builder.new();
    builder.add_counter(ez1, locator.on_end(2, Dir::FacingA, 20.0, Dir::FacingB));
    builder.add_counter(ez1, locator.on_end(11, Dir::FacingB, 20.0, Dir::FacingA));
    builder.add_counter(ez1, locator.on_end(4, Dir::FacingB, 100.0, Dir::FacingA));
    builder.block(ez1);

    let ez2 = builder.new();
    builder.add_counter(ez2, locator.on_end(9, Dir::FacingA, 20.0, Dir::FacingB));
    builder.add_counter(ez2, locator.on_end(11, Dir::FacingA, 20.0, Dir::FacingB));
    builder.add_counter(ez2, locator.on_end(8, Dir::FacingA, 100.0, Dir::FacingB));
    builder.block(ez2);

    builder.flush();
}

fn setup_signals(_: On<ZonesBuilt>, mut builder: SignalBuilder, locator: Locator) {
    let s0 = builder.new(locator.on_end(0, Dir::FacingB, 0.0, Dir::FacingB), -50.0);
    builder.block_entry_signal(s0, 0);

    let s1 = builder.new(locator.on_end(2, Dir::FacingA, 25.0, Dir::FacingB), -50.0);
    builder.block_entry_signal(s1, 1);

    let s2 = builder.new(locator.on_end(8, Dir::FacingA, 110.0, Dir::FacingB), -50.0);
    builder.block_entry_signal(s2, 2);

    let s3 = builder.new(locator.on_end(11, Dir::FacingA, 30.0, Dir::FacingB), -50.0);
    builder.block_entry_signal(s3, 2);
}
