use bevy::ecs::entity::Entity;
use thiserror::Error;

use crate::loc::error::LocError;

#[derive(Debug, Error)]
pub enum LandmarkError {
    #[error("reference to landmark broken: {0}")]
    BrokenLandmarkReference(Entity),

    #[error("problem tracking cursor to next track: {0:?}")]
    CursorError(LocError),
}
