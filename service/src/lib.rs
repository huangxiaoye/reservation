mod service;

use abi::Reservation;
use futures::Stream;
use reservation::ReservationManager;
use std::pin::Pin;
use tonic::Status;

pub struct RsvpService {
    manager: ReservationManager,
}
type ReservationStream = Pin<Box<dyn Stream<Item = Result<Reservation, Status>> + Send>>;
//type ListenStream = Pin<Box<dyn Stream<Item = Result<ListenResponse, Status>> + Send>>;
// impl Deref for RsvpService {
//     type Target = ReservationManager;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
