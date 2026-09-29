use bevy::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocError {
    #[error("reference to segment broken: {0}")]
    BrokenSegmentReference(Entity),

    #[error("reference to node broken: {0}")]
    BrokenNodeReference(Entity),

    #[error("segments[{0}] not in store")]
    SegmentNotInStore(usize),

    #[error("no neighboring track found")]
    NoNeighborSegment,
}
