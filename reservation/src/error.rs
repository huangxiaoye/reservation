use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    #[error("Database error")]
    DbError(#[from] sqlx::Error),
    #[error("invalid start or end time for the reservation")]
    InvalidTime,
    #[error("unknown error")]
    Unknown,
}
