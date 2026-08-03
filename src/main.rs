mod browsing;
mod track;
mod train;

use bevy::{color::palettes::css::BLUE, prelude::*};
use track::*;

use browsing::BrowsingPlugin;

use crate::train::{Direction, Train, TrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TrackPlugin)
        .add_plugins(BrowsingPlugin)
        .add_plugins(TrainPlugin)
        .add_systems(Startup, setup)
        .run();
}

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

    let straight_track_a = commands
        .spawn(TrackSegment::straight((node_x, node_y)))
        .id();
    let curved_track_a = commands
        .spawn(TrackSegment::curved((node_y, node_z), center_z))
        .id();
    let straight_track_b = commands
        .spawn(TrackSegment::straight((node_z, node_b)))
        .id();
    let curved_track_b = commands
        .spawn(TrackSegment::curved((node_b, node_c), center_a))
        .id();
    let straight_track_c = commands
        .spawn(TrackSegment::straight((node_c, node_d)))
        .id();

    commands.trigger(TrackUpdated);

    let trains = [
        Train::on_track(straight_track_a, Direction::Backward).with_speed(1.0),
        Train::on_track(straight_track_b, Direction::Backward).with_speed(1.0),
        Train::on_track(straight_track_c, Direction::Backward).with_speed(1.0),
        Train::on_track(curved_track_a, Direction::Forward).with_speed(1.0),
        Train::on_track(curved_track_b, Direction::Forward).with_speed(1.0),
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
