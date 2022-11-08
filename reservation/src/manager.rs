use sqlx::postgres::types::PgRange;
use sqlx::Row;
use crate::{ Rsvp, ReservationManager, ReservationError, ReservationId };
use chrono::{Utc, DateTime};
use async_trait::async_trait;


#[async_trait]
impl Rsvp for ReservationManager {
  async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError> {
    if rsvp.start.is_none() || rsvp.end.is_none() {
      return Err(ReservationError::InvalidTime);
    }
    // let status = abi::ReservationStatus::from_i32(rsvp.status)
    // .unwrap_or(abi::ReservationStatus::pending);
    let start = abi::convert_to_utc_time(rsvp.start.as_ref().unwrap().clone());

    let end = abi::convert_to_utc_time(rsvp.end.as_ref().unwrap().clone());

    let timespan: PgRange<DateTime<Utc>> = (start..end).into();
    // generate a insert sql for the reservation
    //let sql = "INSERT INTO reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5) RETURNING id";

    // execute the insert sql
    let id = sqlx::query(
      "INSERT INTO reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5) RETURNING id"
    )
    .bind(rsvp.user_id.clone())
    .bind(rsvp.resource_id.clone())
    .bind(timespan)
    .bind(rsvp.note.clone())
    .bind(rsvp.status)
    .fetch_one(&self.pool)
    .await?
    .get(0);
    rsvp.id = id;
    Ok(rsvp)
  }
  async fn change_status(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
    todo!("");
  }

  async fn update_note(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
    todo!("");
  }

  async fn delete(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
    todo!("");
  }

  async fn query(&self, _query: abi::ReservationQuery) -> Result<abi::Reservation, ReservationError> {
    todo!("");
  }

  async fn get(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
    todo!();
  }
}
