mod mouse;
mod render;
mod switch;
mod track;
mod train;

use bevy::{color::palettes::css::BLUE, prelude::*};

use mouse::BrowsingPlugin;
use switch::SwitchPlugin;
use track::*;
use train::{Direction, Train, TrainPlugin};

use render::debug::DebugRenderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TrackPlugin)
        .add_plugins(BrowsingPlugin)
        .add_plugins(TrainPlugin)
        .add_plugins(SwitchPlugin)
        .add_plugins(DebugRenderPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
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
            .spawn(TrackSegment::straight((node_d, node_e)))
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

    commands.trigger(TrackUpdated);

    // spawn train
    for track in tracks {
        commands.spawn((
            Train::on_track(track, Direction::Backward).with_speed(1.0),
            Transform::from_xyz(100.0, 0.0, 0.0),
        ));
    }
}
