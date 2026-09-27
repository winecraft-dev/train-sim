use bevy::ecs::entity::Entity;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrackError {
    #[error("node[{0}] not in store")]
    NodeNotInStore(usize),

    #[error("reference to node broken: {0}")]
    BrokenNodeReference(Entity),
}
