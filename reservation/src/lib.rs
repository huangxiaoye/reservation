mod manager;
use abi::{FilterPager, ReservationId};
use async_trait::async_trait;
use sqlx::PgPool;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}
#[async_trait]
pub trait Rsvp {
    /// make a reservation
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error>;
    /// change a reservation stataus (if current reservation stataus is pending, change it to confirmed)
    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error>;
    /// update note
    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, abi::Error>;
    /// delete reservation
    async fn delete(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error>;
    /// get reservation by id
    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error>;

    /// query reservations
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> mpsc::Receiver<Result<abi::Reservation, abi::Error>>;

    /// query reservations ordered by reservation id
    async fn filter(
        &self,
        filter: abi::ReservationFilter,
    ) -> Result<(FilterPager, Vec<abi::Reservation>), abi::Error>;
}
