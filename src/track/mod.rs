use std::f32::consts::PI;

use bevy::{
    color::palettes::css::RED, ecs::relationship::RelationshipSourceCollection, prelude::*,
};
use track_mesh::TrackMeshBuilder;

mod track_mesh;

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(compute_node_neighbors)
            .add_observer(calculate_track_data)
            .add_observer(generate_track_mesh);
    }
}

#[derive(Event)]
pub struct TrackUpdated;

#[derive(Event)]
pub struct TrackDataCalculated;

#[derive(Event)]
pub struct NodeNeighborsComputed;

#[derive(Debug, Component)]
pub struct TrackNode {
    pub position: Vec2,

    pub neighbors: Vec<Entity>, // neighboring segments
}

impl TrackNode {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            neighbors: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum TrackVariant {
    Straight,
    Curved {
        center: Entity,
        angle: Option<f32>,
        radius: Option<f32>,
    },
}

#[derive(Debug, Component)]
pub struct TrackSegment {
    pub nodes: (Entity, Entity),
    pub variant: TrackVariant,

    pub length: Option<f32>,
    pub node_angles: Option<(f32, f32)>,
}

impl TrackSegment {
    pub fn straight(nodes: (Entity, Entity)) -> Self {
        Self {
            nodes,
            variant: TrackVariant::Straight,

            length: None,
            node_angles: None,
        }
    }

    pub fn curved(nodes: (Entity, Entity), center: Entity) -> Self {
        Self {
            nodes,
            variant: TrackVariant::Curved {
                center,
                angle: Option::None,
                radius: Option::None,
            },

            length: None,
            node_angles: None,
        }
    }

    fn calculate_length(&mut self, nodes: &Query<&TrackNode>) {
        let a = nodes.get(self.nodes.0).unwrap().position;
        let b = nodes.get(self.nodes.1).unwrap().position;

        let length = match &mut self.variant {
            TrackVariant::Straight => (a - b).length(),
            TrackVariant::Curved {
                center,
                angle,
                radius,
            } => {
                let center = nodes.get(*center).unwrap().position;
                let angle_a = (a - center).to_angle();
                let angle_b = (b - center).to_angle();

                let calc_angle = ((angle_b - angle_a + PI) % (2.0 * PI)) - PI;
                let calc_radius = (a - center).length();

                *angle = Some(calc_angle);
                *radius = Some(calc_radius);

                (calc_angle * calc_radius).abs()
            }
        };
        self.length = Some(length);
    }

    fn calculate_node_angles(&mut self, nodes: &Query<&TrackNode>) {
        let a = nodes.get(self.nodes.0).unwrap().position;
        let b = nodes.get(self.nodes.1).unwrap().position;

        // we must precompute the segment's Node Angles
        let node_angles = match self.variant {
            TrackVariant::Straight => {
                let a_angle = (b - a).to_angle();
                let b_angle = (a - b).to_angle();
                (a_angle, b_angle)
            }
            TrackVariant::Curved {
                center,
                angle: _,
                radius: _,
            } => {
                let center = nodes.get(center).unwrap().position;

                let a_angle = (a - center).to_angle();
                let b_angle = (b - center).to_angle();

                let rot_a = a_angle + PI / 2.0;
                let rot_b = b_angle + PI / 2.0;

                (rot_a, rot_b)
            }
        };
        self.node_angles = Some(node_angles);
    }

    pub fn length(&self) -> f32 {
        self.length.unwrap()
    }

    pub fn angle_from(&self, from: Entity) -> Option<f32> {
        let node_angles = self.node_angles.unwrap();
        if self.nodes.0 == from {
            Some(node_angles.0)
        } else if self.nodes.1 == from {
            Some(node_angles.1)
        } else {
            None
        }
    }
}

pub fn compute_node_neighbors(
    _track_updated: On<TrackUpdated>,
    mut commands: Commands,
    mut nodes: Query<&mut TrackNode>,
    segments: Query<(Entity, &TrackSegment)>,
) {
    for (entity, segment) in segments {
        let a = segment.nodes.0;
        let b = segment.nodes.1;
        let segment_nodes = nodes.get_many_mut([a, b]).unwrap();
        for mut s_node in segment_nodes {
            s_node.neighbors.add(entity);
        }
    }
    commands.trigger(NodeNeighborsComputed);
}

pub fn calculate_track_data(
    _track_updated: On<TrackUpdated>,
    mut commands: Commands,
    nodes: Query<&TrackNode>,
    segments: Query<&mut TrackSegment>,
) {
    for mut segment in segments {
        segment.calculate_length(&nodes);
        segment.calculate_node_angles(&nodes);
    }

    commands.trigger(TrackDataCalculated);
}

pub fn generate_track_mesh(
    _track_updated: On<TrackDataCalculated>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    nodes: Query<(Entity, &TrackNode)>,
    segments: Query<&TrackSegment>,
) {
    let mut track_builder = TrackMeshBuilder::default();
    for (entity, node) in nodes {
        track_builder.add_node(entity, node);
    }
    for segment in segments {
        match segment.variant {
            TrackVariant::Straight => track_builder.add_straight_track(segment.nodes),
            TrackVariant::Curved {
                center,
                angle,
                radius,
            } => track_builder.add_curved_track(
                segment.nodes,
                center,
                angle.unwrap(),
                radius.unwrap(),
            ),
        }
    }

    let track_mesh = track_builder.build();
    let track_id = meshes.add(track_mesh.clone());

    commands.spawn((
        Mesh2d(track_id),
        Transform::from_translation(Vec3::ZERO),
        MeshMaterial2d(materials.add(Color::Srgba(RED))),
    ));
}
