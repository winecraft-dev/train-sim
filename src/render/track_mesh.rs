use std::collections::HashMap;

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub struct TrackMeshBuilder {
    curve_resolution: u32,
    // track_width: f32,
    nodes: HashMap<Entity, Vec2>,

    positions: Vec<Vec2>, // outputs
    indices: Vec<usize>,
}

impl Default for TrackMeshBuilder {
    fn default() -> Self {
        Self {
            curve_resolution: 32,
            // track_width: 5.0,
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
impl TrackMeshBuilder {
    pub const fn _curve_resolution(mut self, r: u32) -> Self {
        self.curve_resolution = r;
        self
    }

    // pub const fn track_width(mut self, width: f32) -> Self {
    //     self.track_width = width;
    //     self
    // }

    pub fn add_node(&mut self, entity: Entity, position: Vec2) {
        self.nodes.insert(entity, position);
    }

    pub fn add_straight_track(&mut self, nodes: (Entity, Entity)) {
        let a = self.nodes.get(&nodes.0).unwrap();
        let b = self.nodes.get(&nodes.1).unwrap();

        self.positions.push(*a);
        self.indices.push(self.positions.len() - 1);
        self.positions.push(*b);
        self.indices.push(self.positions.len() - 1);
    }

    pub fn add_curved_track(
        &mut self,
        nodes: (Entity, Entity),
        center: Entity,
        angle: f32,
        radius: f32,
    ) {
        let a = self.nodes.get(&nodes.0).unwrap();
        let center = self.nodes.get(&center).unwrap();

        let angle_a = (a - center).to_angle();

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
        }

        for i in 0..steps - 1 {
            self.indices.push(start_index + i);
            self.indices.push(start_index + i + 1);
        }
    }
}

impl MeshBuilder for TrackMeshBuilder {
    fn build(&self) -> Mesh {
        let positions: Vec<Vec3> = self.positions.iter().map(|p| p.extend(0.0)).collect();
        let indices = self.indices.iter().map(|i| *i as u32).collect();

        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_indices(Indices::U32(indices))
    }
}
