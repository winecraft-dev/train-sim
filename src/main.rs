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
    let node_b = commands.spawn(TrackNode::new(300.0, 0.0)).id();
    let center_a = commands.spawn(TrackNode::new(300.0, 150.0)).id(); // center
    let node_c = commands.spawn(TrackNode::new(450.0, 150.0)).id();
    let node_d = commands.spawn(TrackNode::new(450.0, 600.0)).id();

    commands.spawn(TrackSegment::straight((node_x, node_y)));
    let start_track_z = commands
        .spawn(TrackSegment::curved((node_y, node_z), center_z))
        .id();
    let start_track_a = commands
        .spawn(TrackSegment::straight((node_z, node_b)))
        .id();
    let start_track_b = commands
        .spawn(TrackSegment::curved((node_b, node_c), center_a))
        .id();
    let start_track_c = commands
        .spawn(TrackSegment::straight((node_c, node_d)))
        .id();
    let start_track_d = commands
        .spawn(TrackSegment::straight((node_d, node_x)))
        .id();

    commands.trigger(TrackUpdate);

    let trains = [
        Train::on_track(start_track_z)
            .with_progress(100.0)
            .with_speed(0.3),
        Train::on_track(start_track_a)
            .with_progress(100.0)
            .with_speed(-0.3),
        Train::on_track(start_track_b).with_speed(0.3),
        Train::on_track(start_track_c).with_speed(0.3),
        Train::on_track(start_track_d).with_speed(0.3),
    ];

    // spawn train
    let circle = meshes.add(Circle::new(3.0));
    let blue = materials.add(Color::Srgba(BLUE));

    for train in trains {
        commands.spawn((
            train,
            Mesh2d(circle.clone()),
            MeshMaterial2d(blue.clone()),
            Transform::from_xyz(100.0, 0.0, 0.0),
        ));
    }
}
