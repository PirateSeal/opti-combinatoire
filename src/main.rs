use std::{fs};
use serde_json::from_str;

use model::{airport, flight};

use crate::airport::Airport;
use crate::flight::Flight;

mod model;
mod api;

fn main() {
    let mut airports: Vec<Airport> = load_airports();

    load_flights(&mut airports);

    let airport = airports.iter().find(|airport| airport.code == "CDG").unwrap();

    println!("Airport {:?} {:?}", &airport.code, &airport.name);
    for flight in &airport.flights {
        println!("Flight from {:?} to {:?} on {:?}", flight.origin, flight.destination, flight.departure_time)
    }
}

fn load_airports() -> Vec<Airport> {
    println!("Reading airports data");
    let data = fs::read_to_string("./src/airports.json").expect("Unable to read airports.json");

    println!("Loading airports data");
    let airports: Vec<Airport> = from_str(&data).expect("Unable to parse json");

    airports
}

fn load_flights(airports: &mut Vec<Airport>) {
    println!("Reading flights data");
    let data = fs::read_to_string("./src/data.json").expect("Unable to read flights.json");

    println!("Loading flights data");
    let flights: Vec<Flight> = from_str(&data).expect("Unable to parse json");

    for flight in flights {
        for airport in &mut *airports {
            if flight.origin == airport.code || flight.destination == airport.code && (airport.flights.contains(&flight) || airport.flights.is_empty()) {
                airport.flights.push(flight.clone())
            }
        }
    }
}