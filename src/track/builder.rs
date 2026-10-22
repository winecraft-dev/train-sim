use std::f32::consts::PI;

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, BrokenNodeReference},
    track::{
        TrackNode, TrackSegment, TrackVariant,
        store::{NodeStore, SegmentStore},
    },
};

#[derive(Event)]
pub struct TrackNodesBuilt;

#[derive(Event)]
pub struct TrackBuilt;

#[derive(SystemParam)]
pub struct TrackBuilder<'w, 's> {
    node_store: ResMut<'w, NodeStore>,
    segment_store: ResMut<'w, SegmentStore>,
    commands: Commands<'w, 's>,
    nodes: Query<'w, 's, &'static Transform, With<TrackNode>>,
}

impl<'w, 's> TrackBuilder<'w, 's> {
    pub fn node(&mut self, x: f32, y: f32) -> Entity {
        let e_node = self
            .commands
            .spawn((TrackNode::default(), Transform::from_xyz(x, y, 0.0)))
            .id();
        self.node_store.push(e_node);
        e_node
    }

    pub fn straight(&mut self, a: usize, b: usize) -> Result<Entity, SimError> {
        let ea = self.node_store.node(a)?;
        let eb = self.node_store.node(b)?;

        let a = match self.nodes.get(ea) {
            Ok(transform) => transform.translation.xy(),
            Err(_) => return Err(BrokenNodeReference(ea)),
        };
        let b = match self.nodes.get(eb) {
            Ok(transform) => transform.translation.xy(),
            Err(_) => return Err(BrokenNodeReference(eb)),
        };

        let length = self.calculate_straight_length(a, b);
        let node_angles = self.calculate_straight_node_angles(a, b);

        let e_segment = self
            .commands
            .spawn(TrackSegment {
                nodes: (ea, eb),
                variant: TrackVariant::Straight,

                length,
                node_angles,
            })
            .id();
        self.segment_store.push(e_segment);
        Ok(e_segment)
    }

    pub fn curved(&mut self, a: usize, b: usize, c: usize) -> Result<Entity, SimError> {
        let ea = self.node_store.node(a)?;
        let eb = self.node_store.node(b)?;
        let ec = self.node_store.node(c)?;

        let a = match self.nodes.get(ea) {
            Ok(transform) => transform.translation.xy(),
            Err(_) => return Err(BrokenNodeReference(ea)),
        };
        let b = match self.nodes.get(eb) {
            Ok(transform) => transform.translation.xy(),
            Err(_) => return Err(BrokenNodeReference(eb)),
        };
        let c = match self.nodes.get(ec) {
            Ok(transform) => transform.translation.xy(),
            Err(_) => return Err(BrokenNodeReference(ec)),
        };

        let (angle, radius, length) = self.calculate_curved_data(a, b, c);
        let node_angles = self.calculate_curved_node_angles(a, b, c, angle);

        let e_segment = self
            .commands
            .spawn(TrackSegment {
                nodes: (ea, eb),
                variant: TrackVariant::Curved {
                    center: ec,
                    angle,
                    radius,
                },
                length,
                node_angles,
            })
            .id();

        self.segment_store.push(e_segment);
        Ok(e_segment)
    }

    pub fn nodes_built(&mut self) {
        self.commands.trigger(TrackNodesBuilt);
    }

    pub fn built(&mut self) {
        self.commands.trigger(TrackBuilt);
    }

    fn calculate_straight_length(&mut self, a: Vec2, b: Vec2) -> f32 {
        (a - b).length()
    }

    fn calculate_curved_data(&mut self, a: Vec2, b: Vec2, c: Vec2) -> (f32, f32, f32) {
        let angle_a = (a - c).to_angle();
        let angle_b = (b - c).to_angle();

        let angle = ((angle_b - angle_a + PI) % (2.0 * PI)) - PI;
        let radius = (a - c).length();
        let length = (angle * radius).abs();

        (angle, radius, length)
    }

    fn calculate_straight_node_angles(&mut self, a: Vec2, b: Vec2) -> (f32, f32) {
        let a_angle = (b - a).to_angle();
        let b_angle = (a - b).to_angle();
        (a_angle, b_angle)
    }

    fn calculate_curved_node_angles(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        angle: f32,
    ) -> (f32, f32) {
        let mut a_angle = (a - c).to_angle();
        let mut b_angle = (b - c).to_angle();

        if angle < 0.0 {
            a_angle -= PI / 2.0;
            b_angle += PI / 2.0;
        } else {
            a_angle += PI / 2.0;
            b_angle -= PI / 2.0;
        }

        (a_angle, b_angle)
    }
}
