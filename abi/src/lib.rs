mod pb;
mod error;
mod types;
mod utils;

pub use pb::*;
pub use error::{Error, ReservationConflictInfo, ReservationConflict, ReservationWindow};
pub use utils::*;


// database equivalent of the "reservation_status" enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name="reservation_status", rename_all="lowercase")]
pub enum RsvpStatus {
  Unknown,
  Pending,
  Confirmed,
  Blocked,
}
