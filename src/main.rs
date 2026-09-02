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
    loc::{Direction, FacingLocation, Location, LocationPlugin},
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
        .add_systems(Startup, (config, setup_tracks).chain())
        .add_observer(setup_trains)
        .add_observer(setup_blocks)
        .run();
}

fn config(mut config: ResMut<GizmoConfigStore>, mut commands: Commands) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;

    commands.spawn((Camera2d, Camera::default()));
}

#[derive(Resource)]
pub struct TrackStore {
    nodes: Vec<Entity>,
    segments: Vec<Entity>,
}

fn setup_tracks(mut commands: Commands) {
    let n = [
        TrackNode::spawn(50.0, 0.0, &mut commands),
        TrackNode::spawn(250.0, 0.0, &mut commands),
        TrackNode::spawn(250.0, 100.0, &mut commands), // center
        TrackNode::spawn(350.0, 100.0, &mut commands),
        TrackNode::spawn(250.0, 200.0, &mut commands),
        TrackNode::spawn(150.0, 100.0, &mut commands),
        TrackNode::spawn(50.0, 100.0, &mut commands), // center
        TrackNode::spawn(-50.0, 0.0, &mut commands),
        TrackNode::spawn(-250.0, 0.0, &mut commands),
        TrackNode::spawn(-250.0, -100.0, &mut commands), // center
        TrackNode::spawn(-350.0, -100.0, &mut commands),
        TrackNode::spawn(-250.0, -200.0, &mut commands),
        TrackNode::spawn(-150.0, -100.0, &mut commands),
        TrackNode::spawn(-50.0, -100.0, &mut commands), // center
    ];

    let t = [
        TrackSegment::straight((n[0], n[1])).spawn(&mut commands),
        TrackSegment::curved((n[1], n[3]), n[2]).spawn(&mut commands),
        TrackSegment::curved((n[3], n[4]), n[2]).spawn(&mut commands),
        TrackSegment::curved((n[4], n[5]), n[2]).spawn(&mut commands),
        TrackSegment::curved((n[5], n[0]), n[6]).spawn(&mut commands),
        TrackSegment::straight((n[0], n[7])).spawn(&mut commands),
        TrackSegment::straight((n[8], n[7])).spawn(&mut commands),
        TrackSegment::curved((n[8], n[10]), n[9]).spawn(&mut commands),
        TrackSegment::curved((n[11], n[10]), n[9]).spawn(&mut commands),
        TrackSegment::curved((n[11], n[12]), n[9]).spawn(&mut commands),
        TrackSegment::curved((n[12], n[7]), n[13]).spawn(&mut commands),
    ];

    commands.insert_resource(TrackStore {
        nodes: n.to_vec(),
        segments: t.to_vec(),
    });
    commands.trigger(TrackUpdated);
}

fn setup_trains(_done: On<SwitchesSpawned>, mut commands: Commands, store: Res<TrackStore>) {
    Train::new(2.0).create(&mut commands, Location::new(store.segments[1]));
    // Train::new(1.1).create(&mut commands, Location::new(store.segments[10]));
}

fn setup_blocks(
    _done: On<SwitchesSpawned>,
    mut commands: Commands,
    store: Res<TrackStore>,
    segments: Query<&TrackSegment>,
) {
    let blocks = [
        BlockBuilder::bounds(
            location_at(&store, segments, 5, Direction::FacingB, 0.0),
            location_at(&store, segments, 5, Direction::FacingA, 0.0),
        ),
        BlockBuilder::bounds(
            location_at(&store, segments, 0, Direction::FacingB, 50.0),
            location_at(&store, segments, 1, Direction::FacingA, 0.0),
        ),
        BlockBuilder::bounds(
            location_at(&store, segments, 2, Direction::FacingB, 0.0),
            location_at(&store, segments, 4, Direction::FacingA, 50.0),
        ),
        BlockBuilder::bounds(
            location_at(&store, segments, 6, Direction::FacingA, 50.0),
            location_at(&store, segments, 7, Direction::FacingA, 0.0),
        ),
        BlockBuilder::bounds(
            location_at(&store, segments, 8, Direction::FacingA, 0.0),
            location_at(&store, segments, 10, Direction::FacingA, 50.0),
        ),
    ];

    for block in blocks {
        block.create(&mut commands);
    }
}

fn location_at(
    store: &Res<TrackStore>,
    segments: Query<&TrackSegment>,
    i: usize,
    end: Direction,
    offset: f32,
) -> FacingLocation {
    let e_segment = store.segments[i];
    let distance = match end {
        Direction::FacingA => {
            let segment = segments.get(e_segment).unwrap();
            segment.length() - offset
        }
        Direction::FacingB => 0.0 + offset,
    };
    (Location::new(e_segment).with_distance(distance), end)
}
