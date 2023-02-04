mod vehicle;
mod customer;
mod datastructures;
mod consts;

use std::io::stdin;
use crate::datastructures::{VehicleSet, CustomerMap};
use crate::customer::Customer;
use crate::vehicle::Vehicle;
use crate::consts::*;

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
    
    let mut t = 0;
    let mut customers = CustomerMap::new(5);
    while s != "}" {
        stdin().read_line(&mut s).expect("wot?");
        for line in s.split(";").map(|line| line.trim()) {
            let parts = s.split(",").map(|part| part.trim()).collect();
            match parts[0] {
                "pass" => {
                    t += 1;
                },
                "buy" => {
                    let code = usize::from_str_radix(parts[1])?;
                    let customer = Customer {
                        code: code,
                        tickets: 
                    };
                    customers.add(code, 
                }
            }
        }
        
    }

    println!("h {}", s);
}
