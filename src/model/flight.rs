use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;

use crate::model::airport::Airport;

//struct flight with flight number, departure time, arrival time, price and duration
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Flight {
    pub origin: String,
    pub destination: String,
    pub departure_time: String,
    pub arrival_time: String,
    pub price: u32,
    pub airline: String,
    pub stops: u32
}

impl Flight {
    pub fn new(
        origin: String,
        destination: String,
        departure_time: String,
        arrival_time: String,
        price: u32,
        airline: String,
        stops: u32
    ) -> Flight {
        Flight {
            origin,
            destination,
            departure_time,
            arrival_time,
            price,
            airline,
            stops
        }
    }
}