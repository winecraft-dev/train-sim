mod browsing;
mod switch;
mod track;
mod train;

use std::process::id;

use bevy::{color::palettes::css::BLUE, prelude::*};

use browsing::BrowsingPlugin;
use switch::SwitchPlugin;
use track::*;
use train::{Direction, Train, TrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TrackPlugin)
        .add_plugins(BrowsingPlugin)
        .add_plugins(TrainPlugin)
        .add_plugins(SwitchPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((Camera2d, Camera::default()));

    let center_a = commands.spawn(TrackNode::new(-300.0, 50.0)).id();
    let node_a = commands.spawn(TrackNode::new(-300.0, 0.0)).id();
    let node_b = commands.spawn(TrackNode::new(-350.0, 50.0)).id();
    let node_c = commands.spawn(TrackNode::new(-350.0, 300.0)).id();
    let center_b = commands.spawn(TrackNode::new(-300.0, 300.0)).id();
    let node_d = commands.spawn(TrackNode::new(-300.0, 350.0)).id();
    let node_e = commands.spawn(TrackNode::new(-50.0, 350.0)).id();
    let center_c = commands.spawn(TrackNode::new(-50.0, 300.0)).id();
    let node_f = commands.spawn(TrackNode::new(0.0, 300.0)).id();
    let node_g = commands.spawn(TrackNode::new(0.0, 50.0)).id();
    let center_d = commands.spawn(TrackNode::new(-50.0, 50.0)).id();
    let node_h = commands.spawn(TrackNode::new(-50.0, 0.0)).id();
    let node_far = commands.spawn(TrackNode::new(300.0, 0.0)).id();

    let curved_track_a = commands
        .spawn(TrackSegment::curved((node_a, node_b), center_a))
        .id();
    let straight_track_a = commands
        .spawn(TrackSegment::straight((node_b, node_c)))
        .id();
    let curved_track_b = commands
        .spawn(TrackSegment::curved((node_c, node_d), center_b))
        .id();
    let straight_track_b = commands
        .spawn(TrackSegment::straight((node_d, node_e)))
        .id();
    let curved_track_c = commands
        .spawn(TrackSegment::curved((node_e, node_f), center_c))
        .id();
    let straight_track_c = commands
        .spawn(TrackSegment::straight((node_f, node_g)))
        .id();
    let curved_track_d = commands
        .spawn(TrackSegment::curved((node_g, node_h), center_d))
        .id();
    let straight_track_d = commands
        .spawn(TrackSegment::straight((node_h, node_a)))
        .id();
    // let straight_track_far = commands
    //     .spawn(TrackSegment::straight((node_h, node_far)))
    //     .id();

    commands.trigger(TrackUpdated);

    let trains = [Train::on_track(straight_track_a, Direction::Forward)];

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

fn _legacy_setup(
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
