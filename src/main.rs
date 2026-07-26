mod browsing;
mod track;

use bevy::prelude::*;
use track::*;

use crate::browsing::BrowsingPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TrackPlugin)
        .add_plugins(BrowsingPlugin)
        .add_systems(Startup, setup)
        .run();
}

// could become like a switch... with multiple adjacent tracks (max 3)
// we could toggle its config to switch from track to track. Switches
// are unidirectional

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Camera::default()));

    let node_x = commands.spawn(TrackNode::new(-450.0, -600.0)).id();
    let node_y = commands.spawn(TrackNode::new(-450.0, -150.0)).id();
    let center_z = commands.spawn(TrackNode::new(-300.0, -150.0)).id();
    let node_z = commands.spawn(TrackNode::new(-300.0, 0.0)).id();
    let node_a = commands.spawn(TrackNode::new(0.0, 0.0)).id();
    let node_b = commands.spawn(TrackNode::new(300.0, 0.0)).id();
    let center_a = commands.spawn(TrackNode::new(300.0, 150.0)).id(); // center
    let node_c = commands.spawn(TrackNode::new(450.0, 150.0)).id();
    let node_d = commands.spawn(TrackNode::new(450.0, 600.0)).id();

    commands.spawn(StraightTrackSegment::new((node_x, node_y)));
    commands.spawn(CurvedTrackSegment::new((node_y, node_z), center_z));
    commands.spawn(StraightTrackSegment::new((node_z, node_a)));
    commands.spawn(StraightTrackSegment::new((node_a, node_b)));
    commands.spawn(CurvedTrackSegment::new((node_b, node_c), center_a));
    commands.spawn(StraightTrackSegment::new((node_c, node_d)));

    commands.trigger(TrackUpdate);
}
