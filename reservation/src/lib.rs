mod error;
mod manager;
use sqlx::PgPool;
use async_trait::async_trait;


pub use error::ReservationError;
pub type ReservationId = String;
pub type UserId = String;
pub type ResourceId = String;

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}
#[async_trait]
pub trait  Rsvp {
    /// make a reservation
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    /// change a reservation stataus (if current reservation stataus is pending, change it to confirmed)
    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    /// update note
    async fn update_note(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    /// delete reservation
    async fn delete(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    /// get reservation by id
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;

    /// query reservations
    async fn query(&self, query: abi::ReservationQuery) -> Result<abi::Reservation, ReservationError>;
}
