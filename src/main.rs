mod control;
mod loc;
mod render;
mod signal;
mod track;
mod train;

use bevy::prelude::*;

use crate::{
    control::ControlPlugin,
    loc::{Direction, FacingLocation, Location, LocationPlugin},
    render::debug::DebugRenderPlugin,
    signal::{SignalPlugin, block::create_block, create_signal},
    track::{SwitchesSpawned, TrackNode, TrackPlugin, TrackSegment, TrackUpdated},
    train::{TrainPlugin, create_train},
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
    #[allow(unused)]
    nodes: Vec<Entity>,
    segments: Vec<Entity>,
}

fn setup_tracks(mut commands: Commands) {
    let n = [
        TrackNode::spawn(100.0, 0.0, &mut commands),
        TrackNode::spawn(300.0, 0.0, &mut commands),
        TrackNode::spawn(300.0, 100.0, &mut commands), // center
        TrackNode::spawn(400.0, 100.0, &mut commands),
        TrackNode::spawn(300.0, 200.0, &mut commands),
        TrackNode::spawn(200.0, 100.0, &mut commands),
        TrackNode::spawn(100.0, 100.0, &mut commands), // center
        TrackNode::spawn(-100.0, 0.0, &mut commands),
        TrackNode::spawn(-300.0, 0.0, &mut commands),
        TrackNode::spawn(-300.0, -100.0, &mut commands), // center
        TrackNode::spawn(-400.0, -100.0, &mut commands),
        TrackNode::spawn(-300.0, -200.0, &mut commands),
        TrackNode::spawn(-200.0, -100.0, &mut commands),
        TrackNode::spawn(-100.0, -100.0, &mut commands), // center
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

fn setup_trains(
    _done: On<SwitchesSpawned>,
    mut commands: Commands,
    store: Res<TrackStore>,
    segments: Query<&TrackSegment>,
) {
    create_train(
        &mut commands,
        3.0,
        location_at(&store, segments, 8, Direction::FacingA, 0.0, false),
    );
    create_train(
        &mut commands,
        1.0,
        location_at(&store, segments, 10, Direction::FacingB, 0.0, false),
    );
}

fn setup_blocks(
    _done: On<SwitchesSpawned>,
    mut commands: Commands,
    store: Res<TrackStore>,
    segments: Query<&TrackSegment>,
) {
    let blocks = [
        create_block(
            &mut commands,
            location_at(&store, segments, 5, Direction::FacingB, 0.0, false),
            location_at(&store, segments, 5, Direction::FacingA, 0.0, false),
        ),
        create_block(
            &mut commands,
            location_at(&store, segments, 0, Direction::FacingB, 5.0, false),
            location_at(&store, segments, 1, Direction::FacingA, 5.0, false),
        ),
        create_block(
            &mut commands,
            location_at(&store, segments, 2, Direction::FacingB, 5.0, false),
            location_at(&store, segments, 4, Direction::FacingA, 5.0, false),
        ),
        create_block(
            &mut commands,
            location_at(&store, segments, 6, Direction::FacingA, 5.0, false),
            location_at(&store, segments, 7, Direction::FacingA, 5.0, false),
        ),
        create_block(
            &mut commands,
            location_at(&store, segments, 8, Direction::FacingA, 5.0, false),
            location_at(&store, segments, 10, Direction::FacingA, 5.0, false),
        ),
    ];
    let signals = [
        create_signal(
            &mut commands,
            blocks[0],
            location_at(&store, segments, 4, Direction::FacingA, 5.0, false),
        ),
        create_signal(
            &mut commands,
            blocks[0],
            location_at(&store, segments, 10, Direction::FacingA, 5.0, false),
        ),
        create_signal(
            &mut commands,
            blocks[1],
            location_at(&store, segments, 0, Direction::FacingB, 0.0, false),
        ),
        create_signal(
            &mut commands,
            blocks[2],
            location_at(&store, segments, 1, Direction::FacingA, 0.0, true),
        ),
        create_signal(
            &mut commands,
            blocks[3],
            location_at(&store, segments, 6, Direction::FacingA, 0.0, false),
        ),
        create_signal(
            &mut commands,
            blocks[4],
            location_at(&store, segments, 7, Direction::FacingA, 0.0, true),
        ),
    ];
    println!("{:?} {:?}", blocks, signals);
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
            segment.length() - offset
        }
        Direction::FacingB => 0.0 + offset,
    };
    let facing = match facing_opposite {
        true => end.flip(),
        false => end,
    };
    (Location::new(e_segment).with_distance(distance), facing)
}
