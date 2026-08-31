mod control;
mod loc;
mod render;
mod signal;
mod track;
mod train;

use bevy::prelude::*;

use control::ControlPlugin;
use render::debug::DebugRenderPlugin;
use track::*;
use train::{Train, TrainPlugin};

use crate::{
    loc::{Direction, Location, LocationPlugin},
    signal::{SignalPlugin, block::BlockBuilder},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TrackPlugin)
        .add_plugins(LocationPlugin)
        .add_plugins(TrainPlugin)
        .add_plugins(ControlPlugin)
        .add_plugins(SignalPlugin)
        .add_plugins(DebugRenderPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut config: ResMut<GizmoConfigStore>, mut commands: Commands) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;

    commands.spawn((Camera2d, Camera::default()));

    let center_a = commands.spawn(TrackNode::bundle(-300.0, 50.0)).id();
    let node_a = commands.spawn(TrackNode::bundle(-300.0, 0.0)).id();
    let node_b = commands.spawn(TrackNode::bundle(-350.0, 50.0)).id();
    let node_c = commands.spawn(TrackNode::bundle(-350.0, 300.0)).id();
    let center_b = commands.spawn(TrackNode::bundle(-300.0, 300.0)).id();
    let node_d = commands.spawn(TrackNode::bundle(-300.0, 350.0)).id();
    let node_e = commands.spawn(TrackNode::bundle(-50.0, 350.0)).id();
    let center_c = commands.spawn(TrackNode::bundle(-50.0, 300.0)).id();
    let node_f = commands.spawn(TrackNode::bundle(0.0, 300.0)).id();
    let node_g = commands.spawn(TrackNode::bundle(0.0, 50.0)).id();
    let center_d = commands.spawn(TrackNode::bundle(-50.0, 50.0)).id();
    let node_h = commands.spawn(TrackNode::bundle(-50.0, 0.0)).id();
    let node_far_a = commands.spawn(TrackNode::bundle(300.0, 0.0)).id();
    let center_far = commands.spawn(TrackNode::bundle(-50.0, -50.0)).id();
    let node_far_b = commands.spawn(TrackNode::bundle(0.0, -50.0)).id();

    let tracks = [
        commands
            .spawn(TrackSegment::straight((node_b, node_c)))
            .id(),
        commands
            .spawn(TrackSegment::curved((node_a, node_b), center_a))
            .id(),
        commands
            .spawn(TrackSegment::curved((node_c, node_d), center_b))
            .id(),
        commands
            .spawn(TrackSegment::straight((node_e, node_d)))
            .id(),
        commands
            .spawn(TrackSegment::curved((node_e, node_f), center_c))
            .id(),
        commands
            .spawn(TrackSegment::straight((node_f, node_g)))
            .id(),
        commands
            .spawn(TrackSegment::curved((node_g, node_h), center_d))
            .id(),
        commands
            .spawn(TrackSegment::straight((node_h, node_a)))
            .id(),
        commands
            .spawn(TrackSegment::straight((node_h, node_far_a)))
            .id(),
        commands
            .spawn(TrackSegment::curved((node_h, node_far_b), center_far))
            .id(),
    ];

    let location = Location::new(tracks[1]);
    commands.trigger(TrackUpdated);

    Train::new(1.0).create(&mut commands, location);

    BlockBuilder::bounds(
        (Location::new(tracks[2]), Direction::FacingB),
        (
            Location::new(tracks[3]).with_distance(150.0),
            Direction::FacingA,
        ),
    )
    .create(commands)
    .unwrap();
}
