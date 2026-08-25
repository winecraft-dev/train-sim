use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignalError {
    #[error("end was not reachable from the start")]
    BoundNotReachable,
    #[error("bound not found")]
    BoundsNotFound,
}
