mod pb;
use chrono::{Utc, DateTime, NaiveDateTime};
pub use pb::*;
use prost_types::Timestamp;

pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {

  DateTime::<Utc>::from_utc(
    NaiveDateTime::from_timestamp(ts.seconds, ts.nanos as _),
    Utc,
  )
}
