use std::{collections::HashMap, f32::consts::PI};

use crate::{CurvedTrackSegment, StraightTrackSegment, TrackNode};

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub struct TrackBuilder {
    curve_resolution: u32,
    track_width: f32,

    nodes: HashMap<Entity, Vec2>,

    positions: Vec<Vec2>, // outputs
    indices: Vec<usize>,
}

impl Default for TrackBuilder {
    fn default() -> Self {
        Self {
            curve_resolution: 10,
            track_width: 5.0,

            nodes: Default::default(),

            positions: Default::default(),
            indices: Default::default(),
        }
    }
}

// we can process as we go for the segments that are straight, but the curved
// tracks need information about the adjacent segments. Since order is not
// guaranteed, we should process curveds after all segments come in.

//
impl TrackBuilder {
    pub const fn curve_resolution(mut self, r: u32) -> Self {
        self.curve_resolution = r;
        self
    }

    pub const fn track_width(mut self, width: f32) -> Self {
        self.track_width = width;
        self
    }

    pub fn add_node(&mut self, entity: Entity, node: &TrackNode) {
        self.nodes.insert(entity, node.position);
    }

    pub fn add_straight_track(&mut self, track: &StraightTrackSegment) {
        let a = self.nodes.get(&track.nodes.0).unwrap();
        let b = self.nodes.get(&track.nodes.1).unwrap();

        let delta = a - b;
        let scaled = delta.normalize() * (self.track_width / 2.0);
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

        self.indices.push(la);
        self.indices.push(lb);
        self.indices.push(ra);
        self.indices.push(rb);
    }

    pub fn add_curved_track(&mut self, track: &CurvedTrackSegment) {
        let a = self.nodes.get(&track.nodes.0).unwrap();
        let b = self.nodes.get(&track.nodes.1).unwrap();
        let center = self.nodes.get(&track.center).unwrap();

        let angle_a = (a - center).to_angle();
        let angle_b = (b - center).to_angle();
        let angle = angle_b - angle_a;
        let radius = (a - center).length();

        let steps = self.curve_resolution as usize; // (((angle / 2.0 / PI) * self.resolution as f32) as usize).min(1);
        let step = angle / (steps as f32 - 1.0);

        let start_index = self.positions.len();
        for i in 0..steps {
            let theta = angle_a + i as f32 * step;
            let (sin, cos) = ops::sin_cos(theta);
            let x = cos * radius;
            let y = sin * radius;
            let position = Vec2::new(x, y) + center;
            self.positions.push(position);

            println!("{} {}", position, self.positions.len() - 1);
        }

        for i in 0..steps - 1 {
            let a = start_index + i;
            let b = start_index + i + 1;
            println!("a: {} b: {}", a, b);
            self.indices.push(start_index + i);
            self.indices.push(start_index + i + 1);
        }
    }
}

impl MeshBuilder for TrackBuilder {
    fn build(&self) -> Mesh {
        let positions: Vec<Vec3> = self.positions.iter().map(|p| p.extend(0.0)).collect();
        let indices = self.indices.iter().map(|i| *i as u32).collect();

        println!("{:?}", positions);
        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_indices(Indices::U32(indices))
    }
}
