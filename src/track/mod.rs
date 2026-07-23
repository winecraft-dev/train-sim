use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};
use track_mesh::TrackMeshBuilder;

mod track_mesh;

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(generate_track_mesh)
            .add_observer(calculate_track_data);
    }
}

#[derive(Event)]
pub struct TrackUpdate;

#[derive(Debug, Component)]
pub struct TrackNode {
    position: Vec2,
}

impl TrackNode {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
        }
    }
}

#[derive(Debug, Component)]
pub struct StraightTrackSegment {
    nodes: (Entity, Entity),
}

impl StraightTrackSegment {
    pub fn new(nodes: (Entity, Entity)) -> Self {
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
    pub fn new(nodes: (Entity, Entity), center: Entity) -> Self {
        Self {
            nodes,
            center,
            radius: None,
        }
    }

    fn calculate_radius(&mut self, nodes: &Query<&TrackNode>) {
        let a = nodes.get(self.nodes.0).unwrap().position;
        let center = nodes.get(self.center).unwrap().position;
        let radius = (a - center).length();

        self.radius = Some(radius);
    }
}

pub fn calculate_track_data(
    _track_updated: On<TrackUpdate>,
    nodes: Query<&TrackNode>,
    curved_segments: Query<&mut CurvedTrackSegment>,
) {
    for mut segment in curved_segments {
        segment.calculate_radius(&nodes);
    }
}

pub fn generate_track_mesh(
    _track_updated: On<TrackUpdate>,
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

    let mut track_builder = TrackMeshBuilder::default();
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
