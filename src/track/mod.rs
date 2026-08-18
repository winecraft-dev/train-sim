use std::f32::consts::PI;

use bevy::{ecs::relationship::RelationshipSourceCollection, prelude::*};

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(compute_node_neighbors)
            .add_observer(calculate_track_data);
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
    pub neighbors: Vec<Entity>, // neighboring segments
}

impl TrackNode {
    pub fn bundle(x: f32, y: f32) -> impl Bundle {
        (
            Self {
                neighbors: Vec::new(),
            },
            Transform::from_xyz(x, y, 0.0),
        )
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

    fn calculate_length(&mut self, nodes: Query<&Transform, With<TrackNode>>) {
        let a = nodes.get(self.nodes.0).unwrap().translation.xy();
        let b = nodes.get(self.nodes.1).unwrap().translation.xy();

        let length = match &mut self.variant {
            TrackVariant::Straight => (a - b).length(),
            TrackVariant::Curved {
                center,
                angle,
                radius,
            } => {
                let center = nodes.get(*center).unwrap().translation.xy();
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

    fn calculate_node_angles(&mut self, nodes: Query<&Transform, With<TrackNode>>) {
        let a = nodes.get(self.nodes.0).unwrap().translation.xy();
        let b = nodes.get(self.nodes.1).unwrap().translation.xy();

        // we must precompute the segment's Node Angles
        let node_angles = match self.variant {
            TrackVariant::Straight => {
                let a_angle = (b - a).to_angle();
                let b_angle = (a - b).to_angle();
                (a_angle, b_angle)
            }
            TrackVariant::Curved {
                center,
                angle,
                radius: _,
            } => {
                let center = nodes.get(center).unwrap().translation.xy();
                let delta_angle = angle.unwrap();

                let mut a_angle = (a - center).to_angle();
                let mut b_angle = (b - center).to_angle();

                if delta_angle < 0.0 {
                    a_angle -= PI / 2.0;
                    b_angle += PI / 2.0;
                } else {
                    a_angle += PI / 2.0;
                    b_angle -= PI / 2.0;
                }

                (a_angle, b_angle)
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

    pub fn opposite(&self, from: Entity) -> Option<Entity> {
        if self.nodes.0 == from {
            Some(self.nodes.1)
        } else if self.nodes.1 == from {
            Some(self.nodes.0)
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
    nodes: Query<&Transform, With<TrackNode>>,
    segments: Query<&mut TrackSegment>,
) {
    for mut segment in segments {
        segment.calculate_length(nodes);
        segment.calculate_node_angles(nodes);
    }

    commands.trigger(TrackDataCalculated);
}
