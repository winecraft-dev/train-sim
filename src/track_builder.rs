use std::collections::HashMap;

use super::{TrackNode, TrackSegment, TrackType};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

#[derive(Default)]
pub struct TrackBuilder {
    positions: Vec<Vec2>, // outputs
    indices: Vec<u32>,

    nodes: HashMap<Entity, Vec2>,
    curveds: Vec<TrackSegment>,
}

// we can process as we go for the segments that are straight, but the curved
// tracks need information about the adjacent segments. Since order is not
// guaranteed, we should process curveds after all segments come in.

const TRACK_WIDTH: f32 = 5.0;
impl TrackBuilder {
    pub fn add_node(&mut self, entity: Entity, node: &TrackNode) {
        self.nodes.insert(entity, node.position);
    }

    pub fn add_segment(&mut self, segment: &TrackSegment) {
        match segment.track_type {
            TrackType::Straight => self.add_straight_track(segment.nodes),
            TrackType::Curved => self.add_curved_track(segment.nodes),
        }
    }

    fn add_straight_track(&mut self, nodes: (Entity, Entity)) {
        let a = self.nodes.get(&nodes.0).unwrap();
        let b = self.nodes.get(&nodes.1).unwrap();

        println!("{} {} {} {}", nodes.0, nodes.1, a, b);

        let delta = a - b;
        let scaled = delta.normalize() * (TRACK_WIDTH / 2.0);
        let rotated = Vec2::new(scaled.y, -scaled.x); // rotate by pi/2

        let l_track_pos_a = a + rotated;
        let l_track_pos_b = b + rotated;
        let r_track_pos_a = a - rotated;
        let r_track_pos_b = b - rotated;

        self.positions.push(l_track_pos_a);
        let la = self.positions.len() - 1;
        self.positions.push(l_track_pos_b);
        let lb = self.positions.len() - 1;
        self.positions.push(r_track_pos_a);
        let ra = self.positions.len() - 1;
        self.positions.push(r_track_pos_b);
        let rb = self.positions.len() - 1;

        self.indices.push(la as u32);
        self.indices.push(lb as u32);
        self.indices.push(ra as u32);
        self.indices.push(rb as u32);
    }

    fn add_curved_track(&mut self, nodes: (Entity, Entity)) {} // the curved track needs to know information about the approaching tracks...
}

impl MeshBuilder for TrackBuilder {
    fn build(&self) -> Mesh {
        let positions: Vec<Vec3> = self.positions.iter().map(|p| p.extend(0.0)).collect();
        let indices = Indices::U32(self.indices.clone());

        println!("{:?} {:?}", positions, indices);
        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_indices(indices)
    }
}
