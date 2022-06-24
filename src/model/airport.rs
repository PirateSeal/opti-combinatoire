use serde::Deserialize;

use crate::model::flight::Flight;

//struct airport with name, location, and code
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Airport {
    pub name: String,
    pub country_code: String,
    pub region_code: String,
    pub location: String,
    pub code: String,
    pub flights: Vec<Flight>,
}

impl Airport {
    pub fn new(
        name: String,
        location: String,
        code: String,
        country_code: String,
        region_code: String,
        flights: Vec<Flight>
    ) -> Airport {
        Airport {
            name,
            location,
            code,
            country_code,
            region_code,
            flights,
        }
    }

    pub fn get_incoming(&self) -> Vec<&Flight> {
        let mut incoming: Vec<&Flight> = Vec::new();

        for flight in &self.flights {
            if flight.destination == self.code {
                incoming.push(flight);
            }
        }

        incoming
    }

    pub fn get_outgoing(&self) -> Vec<&Flight> {
        let mut outgoing: Vec<&Flight> = Vec::new();

        for flight in &self.flights {
            if flight.origin == self.code {
                outgoing.push(flight)
            }
        }

        outgoing
    }
}