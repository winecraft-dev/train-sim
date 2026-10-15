use bevy::ecs::entity::Entity;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimError {
    #[error("reference to landmark broken: {0}")]
    BrokenLandmarkReference(Entity),

    #[error("reference to segment broken: {0}")]
    BrokenSegmentReference(Entity),

    #[error("reference to node broken: {0}")]
    BrokenNodeReference(Entity),

    #[error("segments[{0}] not in store")]
    SegmentNotInStore(usize),

    #[error("node[{0}] not in store")]
    NodeNotInStore(usize),

    #[error("no neighboring track found")]
    NoNeighborSegment,

    #[error("node[{0}] not of segment[{1}]")]
    NodeNotOfSegment(Entity, Entity),

    #[error("invalid signal location: {0:?}")]
    InvalidSignalLoc(Box<SimError>),

    #[error("invalid observable bound location: {0:?}")]
    InvalidObservableBoundLoc(Box<SimError>),

    #[error("conversion from wire format failed: {0:?}")]
    WireConversionFailed(Box<SimError>),
}
