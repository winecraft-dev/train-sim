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

use crate::{
    control::ControlPlugin, landmark::LandmarkPlugin, loader::LoaderPlugin, loc::LocationPlugin,
    render::debug::DebugRenderPlugin, signal::SignalPlugin, track::TrackPlugin, train::TrainPlugin,
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
        // .add_observer(setup_trains)
        .run();
}

fn config(mut config: ResMut<GizmoConfigStore>, mut commands: Commands) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;

    commands.spawn((Camera2d, Camera::default()));
}

// fn setup_trains(_done: On<SwitchesSpawned>, mut commands: Commands, locator: Locator) {
//     let locs = [
//         locator.on_progress(0, 0.5, Dir::B),
//         locator.on_progress(1, 0.5, Dir::B),
//         locator.on_progress(2, 0.5, Dir::B),
//         locator.on_progress(3, 0.5, Dir::B),
//         locator.on_progress(4, 0.5, Dir::B),
//         locator.on_progress(5, 0.5, Dir::B),
//         locator.on_progress(6, 0.5, Dir::B),
//         locator.on_progress(7, 0.5, Dir::B),
//         locator.on_progress(8, 0.5, Dir::B),
//     ];

//     for loc in locs {
//         let mut rng = rand::rng();
//         create_train(&mut commands, rng.random_range(1.0..2.0), loc.unwrap());
//     }
// }

// fn setup_blocks(_done: On<SwitchesSpawned>, mut builder: ZoneBuilder, locator: Locator) {
//     ZoneConstructor::new()
//         .with_entry(locator.on_end(0, Dir::B, 10.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(0, Dir::B, 200.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(0, Dir::B, 210.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(0, Dir::B, 400.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(0, Dir::B, 410.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(0, Dir::A, 10.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(1, Dir::B, 0.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(1, Dir::A, 0.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(2, Dir::B, 0.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(2, Dir::B, 200.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(2, Dir::A, 20.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(11, Dir::B, 0.0, Dir::A).unwrap())
//         .with_exit(locator.on_end(4, Dir::B, 100.0, Dir::A).unwrap())
//         .with_switch(4)
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(11, Dir::B, 10.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(11, Dir::B, 200.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(11, Dir::B, 210.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(11, Dir::B, 400.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(11, Dir::B, 410.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(11, Dir::A, 10.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_exit(locator.on_end(9, Dir::A, 40.0, Dir::B).unwrap())
//         .with_entry(locator.on_end(8, Dir::A, 100.0, Dir::B).unwrap())
//         .with_entry(locator.on_end(11, Dir::A, 0.0, Dir::B).unwrap())
//         .with_switch(15)
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(9, Dir::A, 60.0, Dir::A).unwrap())
//         .with_exit(locator.on_end(9, Dir::B, 0.0, Dir::B).unwrap())
//         .with_signal_distance(15.0)
//         .with_observable_distance(20.0)
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(12, Dir::B, 0.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(12, Dir::A, 0.0, Dir::A).unwrap())
//         .with_observable_distance(20.0)
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(4, Dir::B, 100.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(4, Dir::A, 0.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(5, Dir::B, 0.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(5, Dir::A, 0.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(6, Dir::B, 10.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(6, Dir::B, 200.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(6, Dir::B, 210.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(6, Dir::B, 400.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(6, Dir::B, 410.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(6, Dir::A, 10.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(7, Dir::B, 0.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(7, Dir::A, 0.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     ZoneConstructor::new()
//         .with_entry(locator.on_end(8, Dir::B, 0.0, Dir::B).unwrap())
//         .with_exit(locator.on_end(8, Dir::A, 110.0, Dir::A).unwrap())
//         .build(&mut builder)
//         .unwrap();

//     builder.done();
// }
