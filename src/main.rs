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

// could become like a switch... with multiple adjacent tracks (max 3)
// we could toggle its config to switch from track to track. Switches
// are unidirectional
#[derive(Debug, Component)]
pub struct TrackNode {
    position: Vec2,
}

impl TrackNode {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
        }
    }
}

#[derive(Debug)]
pub enum TrackType {
    Straight,
    Curved,
}

#[derive(Debug, Component)]
pub struct StraightTrackSegment {
    nodes: (Entity, Entity),
}

impl StraightTrackSegment {
    fn new(nodes: (Entity, Entity)) -> Self {
        Self { nodes }
    }
}

// We should precompute radius before constructing CurvedTrackSegment.
// When we setup track creation, we can create a system for generating
// a radius from two nodes using the triangle method. Radius is tied
// to a lot of things, like drawing and train behavior, so it's worth
// precomputing
#[derive(Debug, Component)]
pub struct CurvedTrackSegment {
    nodes: (Entity, Entity),
    center: Entity, // can derive radius from positions of center and a node

    radius: Option<f32>,
}

impl CurvedTrackSegment {
    fn new(nodes: (Entity, Entity), center: Entity) -> Self {
        Self {
            nodes,
            center,
            radius: None,
        }
    }
}

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
}

fn gen_track_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    nodes: Query<(Entity, &TrackNode)>,
    straight_segments: Query<&StraightTrackSegment>,
    curved_segments: Query<&CurvedTrackSegment>,
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
    for segment in straight_segments {
        track_builder.add_straight_track(segment);
    }
    for segment in curved_segments {
        track_builder.add_curved_track(segment);
    }

    let track_mesh = track_builder.build();
    let track_id = meshes.add(track_mesh.clone());

    commands.spawn((
        Mesh2d(track_id),
        Transform::from_translation(Vec3::ZERO),
        MeshMaterial2d(materials.add(Color::Srgba(RED))),
    ));
}
