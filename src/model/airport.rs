use crate::model::flight::Flight;

//struct airport with name, location, and code
pub struct Airport {
    pub name: String,
    pub location: String,
    pub code: String,
    pub flights: Vec<Flight>,
}

impl Airport {
    pub fn new(name: String, location: String, code: String, flights: Vec<Flight>) -> Airport {
        Airport {
            name,
            location,
            code,
            flights,
        }
    }

    pub fn get_incoming(&self) -> Vec<&Flight> {
        let mut incoming: Vec<&Flight> = Vec::new();

        for flight in &self.flights {
            if flight.destination.code == self.code {
                incoming.push(flight);
            }
        }

        incoming
    }

    pub fn get_outgpoing(&self) -> Vec<&Flight> {
        let mut outgoing: Vec<&Flight> = Vec::new();

        for flight in &self.flights {
            if flight.origin.code == self.code {
                outgoing.push(flight)
            }
        }

        outgoing
    }
}