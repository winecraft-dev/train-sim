mod control;
mod landmark;
mod loc;
mod render;
mod signal;
mod track;
mod train;

use bevy::prelude::*;

use crate::{
    control::ControlPlugin,
    landmark::LandmarkPlugin,
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
        .add_plugins(LandmarkPlugin)
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
        Vec2::new(-300.0, 300.0),
        Vec2::new(300.0, 300.0),
        Vec2::new(300.0, 250.0), // CENTER
        Vec2::new(350.0, 250.0),
        Vec2::new(350.0, 0.0),
        Vec2::new(300.0, 0.0), // CENTER
        Vec2::new(300.0, -50.0),
        Vec2::new(350.0, -250.0),
        Vec2::new(300.0, -250.0), // CENTER
        Vec2::new(300.0, -300.0),
        Vec2::new(-300.0, -300.0),
        Vec2::new(-300.0, -250.0), // CENTER
        Vec2::new(-350.0, -250.0),
        Vec2::new(-300.0, -50.0),
        Vec2::new(-300.0, 0.0), // CENTER
        Vec2::new(-350.0, 0.0),
        Vec2::new(-300.0, 250.0), // CENTER
        Vec2::new(-350.0, 250.0),
    ];

    let commands = &mut commands;

    let n: Vec<Entity> = n
        .iter()
        .map(|p| TrackNode::spawn(p.x, p.y, commands))
        .collect();

    let t = [
        TrackSegment::straight(n[0], n[1]).spawn(commands),
        TrackSegment::curved(n[1], n[3], n[2]).spawn(commands),
        TrackSegment::straight(n[3], n[4]).spawn(commands),
        TrackSegment::curved(n[4], n[6], n[5]).spawn(commands),
        TrackSegment::straight(n[4], n[7]).spawn(commands),
        TrackSegment::curved(n[7], n[9], n[8]).spawn(commands),
        TrackSegment::straight(n[9], n[10]).spawn(commands),
        TrackSegment::curved(n[10], n[12], n[11]).spawn(commands),
        TrackSegment::straight(n[12], n[15]).spawn(commands), // 60
        TrackSegment::straight(n[17], n[15]).spawn(commands), // 61
        TrackSegment::curved(n[13], n[15], n[14]).spawn(commands), // 62
        TrackSegment::straight(n[6], n[13]).spawn(commands),
        TrackSegment::curved(n[17], n[0], n[16]).spawn(commands),
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
        2.0,
        location_at(&store, segments, 1, Direction::FacingA, 50.0, false),
    );
}

fn setup_blocks(
    _done: On<SwitchesSpawned>,
    mut commands: Commands,
    store: Res<TrackStore>,
    segments: Query<&TrackSegment>,
) {
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
