mod browsing;
mod track;
mod train;

use bevy::{color::palettes::css::BLUE, prelude::*};
use track::*;

use browsing::BrowsingPlugin;

use crate::train::{Train, TrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TrackPlugin)
        .add_plugins(BrowsingPlugin)
        .add_plugins(TrainPlugin)
        .add_systems(Startup, setup)
        .run();
}

// could become like a switch... with multiple adjacent tracks (max 3)
// we could toggle its config to switch from track to track. Switches
// are unidirectional

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
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

    commands.spawn(TrackSegment::straight((node_x, node_y)));
    commands.spawn(TrackSegment::curved((node_y, node_z), center_z));
    let start_track = commands
        .spawn(TrackSegment::straight((node_z, node_a)))
        .id();
    commands.spawn(TrackSegment::straight((node_a, node_b)));
    commands.spawn(TrackSegment::curved((node_b, node_c), center_a));
    commands.spawn(TrackSegment::straight((node_c, node_d)));

    commands.trigger(TrackUpdate);

    // spawn train
    let circle = meshes.add(Circle::new(3.0));
    let blue = materials.add(Color::Srgba(BLUE));
    commands.spawn((
        Train::on_track(start_track),
        Mesh2d(circle),
        MeshMaterial2d(blue),
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
}
