use bevy::prelude::*;

use crate::error::SimError::{self, NodeNotInStore, SegmentNotInStore};

#[derive(Resource, Default, DerefMut, Deref, Debug)]
pub struct NodeStore(Vec<Entity>);

#[derive(Resource, Default, DerefMut, Deref, Debug)]
pub struct SegmentStore(Vec<Entity>);

impl NodeStore {
    pub fn init(mut commands: Commands) {
        commands.insert_resource(Self::default());
    }

    pub fn reset(commands: &mut Commands, mut store: ResMut<Self>) {
        for e_node in store.iter() {
            commands.entity(*e_node).despawn();
        }
        store.clear();
    }

    pub fn node(&self, i: usize) -> Result<Entity, SimError> {
        if i < self.len() {
            return Ok(self[i]);
        }
        Err(NodeNotInStore(i))
    }
}

impl SegmentStore {
    pub fn init(mut commands: Commands) {
        commands.insert_resource(Self::default());
    }

    pub fn reset(commands: &mut Commands, mut store: ResMut<Self>) {
        for e_segment in store.iter() {
            commands.entity(*e_segment).despawn();
        }
        store.clear();
    }

    pub fn segment(&self, i: usize) -> Result<Entity, SimError> {
        if i < self.len() {
            return Ok(self[i]);
        }
        Err(SegmentNotInStore(i))
    }
}
