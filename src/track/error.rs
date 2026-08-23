use bevy::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrackError {
    #[error("reference to segment broken: {0}")]
    BrokenSegmentReference(Entity),

    #[error("reference to node broken: {0}")]
    BrokenNodeReference(Entity),

    #[error("no neighbor resolved")]
    NoNeighborSegment,
}
