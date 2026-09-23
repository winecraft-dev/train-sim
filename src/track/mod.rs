use bevy::prelude::*;

use crate::track::{builder::init_track_store, switch::SwitchPlugin};

pub mod builder;
pub mod switch;

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SwitchPlugin)
            .add_systems(PreStartup, init_track_store)
            .add_observer(compute_node_neighbors);
    }
}

#[derive(Event)]
pub struct TrackUpdated;

#[derive(Event)]
pub struct NodeNeighborsComputed;

#[derive(Event)]
pub struct SwitchesSpawned;

#[derive(Debug, Default, Component)]
pub struct TrackNode {
    pub neighbors: Vec<Entity>, // neighboring segments
}

#[derive(Debug)]
pub enum TrackVariant {
    Straight,
    Curved {
        center: Entity,
        angle: f32,
        radius: f32,
    },
}

#[derive(Debug, Component)]
pub struct TrackSegment {
    pub nodes: (Entity, Entity),
    pub variant: TrackVariant,

    pub length: f32,
    pub node_angles: (f32, f32),
}

impl TrackSegment {
    pub fn angle_from(&self, from: Entity) -> Option<f32> {
        if self.nodes.0 == from {
            Some(self.node_angles.0)
        } else if self.nodes.1 == from {
            Some(self.node_angles.1)
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
            s_node.neighbors.push(entity);
        }
    }
    commands.trigger(NodeNeighborsComputed);
}
