use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::model::airport::Airport;

//struct flight with flight number, departure time, arrival time, price and duration
pub struct Flight {
    pub origin: Airport,
    pub destination: Airport,
    pub departure_time: DateTime<Utc>,
    pub arrival_time: DateTime<Utc>,
    pub price: u32,
    pub airline: String
}

impl Flight {
    pub fn new(
        origin: Airport,
        destination: Airport,
        departure_time: DateTime<Utc>,
        arrival_time: DateTime<Utc>,
        price: u32,
        airline: String
    ) -> Flight {
        Flight {
            origin,
            destination,
            departure_time,
            arrival_time,
            price,
            airline
        }
    }

    pub fn get_length(&self) -> Duration {
       return self.arrival_time - self.departure_time
    }
}