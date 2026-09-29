use thiserror::Error;

use crate::loc::error::LocError;

#[derive(Error, Debug)]
pub enum ZoneError {
    #[error("invalid signal location: {0:?}")]
    InvalidSignalLoc(LocError),

    #[error("invalid observable location: {0:?}")]
    InvalidObservableLoc(LocError),
}
