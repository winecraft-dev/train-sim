mod control;
mod landmark;
mod loc;
mod render;
mod signal;
mod track;
mod train;
mod zone;

use bevy::prelude::*;

use crate::{
    control::ControlPlugin,
    landmark::LandmarkPlugin,
    loc::{Direction, FacingLocation, Location, LocationPlugin},
    render::debug::DebugRenderPlugin,
    signal::{SignalPlugin, block::create_block, create_signal},
    track::{
        SwitchesSpawned, TrackNode, TrackPlugin, TrackSegment, TrackUpdated,
        builder::{TrackBuilder, TrackStore},
    },
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
        .add_plugins(DebugRenderPlugin)
        .add_plugins(AxleCounterPlugin)
        .add_systems(Startup, (config, setup_nodes, setup_tracks).chain())
        .add_observer(setup_trains)
        .run();
}

fn config(mut config: ResMut<GizmoConfigStore>, mut commands: Commands) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;

    commands.spawn((Camera2d, Camera::default()));
    commands.insert_resource(TrackStore::default());
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

fn setup_trains(
    _done: On<SwitchesSpawned>,
    mut commands: Commands,
    store: Res<TrackStore>,
    segments: Query<&TrackSegment>,
) {
    create_train(
        &mut commands,
        2.0,
        location_at(&store, segments, 1, Direction::FacingA, 50.0, false),
    );
}

fn location_at(
    store: &Res<TrackStore>,
    segments: Query<&TrackSegment>,
    i: usize,
    end: Direction,
    offset: f32,
    facing_opposite: bool,
) -> FacingLocation {
    let e_segment = store.segments[i];
    let distance = match end {
        Direction::FacingA => {
            let segment = segments.get(e_segment).unwrap();
            segment.length - offset
        }
        Direction::FacingB => 0.0 + offset,
    };
    let facing = match facing_opposite {
        true => end.flip(),
        false => end,
    };
    (Location::new(e_segment).with_distance(distance), facing)
}
