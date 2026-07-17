mod track_builder;

use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};
use track_builder::TrackBuilder;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, gen_track_mesh).chain())
        .run();
}

#[derive(Debug, Component)]
pub struct TrackNode {
    position: Vec2,
    adjacent_tracks: Vec<Entity>,
}

impl TrackNode {
    fn new(position: Vec2) -> Self {
        Self {
            position,
            adjacent_tracks: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum TrackType {
    Straight,
    Curved,
}

#[derive(Debug, Component)]
pub struct TrackSegment {
    nodes: (Entity, Entity),
    track_type: TrackType,
}

impl TrackSegment {
    fn straight(nodes: (Entity, Entity)) -> Self {
        TrackSegment {
            nodes,
            track_type: TrackType::Straight,
        }
    }

    fn curved(nodes: (Entity, Entity)) -> Self {
        TrackSegment {
            nodes,
            track_type: TrackType::Curved,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Camera::default()));

    let node_a = commands.spawn(TrackNode::new(Vec2::new(0.0, 0.0))).id();
    let node_b = commands.spawn(TrackNode::new(Vec2::new(30.0, 30.0))).id();
    let node_c = commands.spawn(TrackNode::new(Vec2::new(60.0, 30.0))).id();
    let node_d = commands.spawn(TrackNode::new(Vec2::new(60.0, 100.0))).id();

    commands.spawn(TrackSegment::straight((node_a, node_b)));
    commands.spawn(TrackSegment::curved((node_b, node_c)));
    commands.spawn(TrackSegment::straight((node_c, node_d)));
}

fn gen_track_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    nodes: Query<(Entity, &TrackNode)>,
    segments: Query<&TrackSegment>,
) {
    let center = meshes.add(Circle::new(2.0));
    commands.spawn((
        Mesh2d(center),
        Transform::from_translation(Vec3::new(-1.0, 1.0, 0.0)),
        MeshMaterial2d(materials.add(Color::Srgba(GREEN))),
    ));

    let mut track_builder = TrackBuilder::default();
    for (entity, node) in nodes {
        track_builder.add_node(entity, node);
    }
    for segment in segments {
        track_builder.add_segment(segment);
    }

    let track_mesh = track_builder.build();
    let track_id = meshes.add(track_mesh.clone());

    println!("{:?}", track_mesh);

    commands.spawn((
        Mesh2d(track_id),
        Transform::from_translation(Vec3::ZERO),
        MeshMaterial2d(materials.add(Color::Srgba(RED))),
    ));
}
