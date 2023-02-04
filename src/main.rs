mod vehicle;
mod customer;
mod datastructures;
mod consts;

use std::io::stdin;
use crate::datastructures::{VehicleSet, CustomerQueue};
use crate::customer::Customer;
use crate::vehicle::Vehicle;

fn main() {
    let mut s = String::new();
    let mut vehicles = VehicleSet::new();
    let mut number_of_vehicles: usize = 0;
    while s != "{" {
        stdin().read_line(&mut s).expect("wot?");
        if let Err(error) = vehicles.add(Vehicle::new_from_string(s.clone(), number_of_vehicles)) {
            println!("Error: {}", error);
        } else {
            println!("Added");
        }
        number_of_vehicles += 1;
    }

    while s != "}" {
        let customers = CustomerQueue::new(100);
        stdin().read_line(&mut s).expect("wot?");
        
    }

    println!("h {}", s);
}
