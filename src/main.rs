mod vehicle;
mod customer;
mod datastructures;
mod consts;

use std::io::stdin;
use crate::datastructures::{VehicleSet, CustomerMap};
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
    
    let mut t = 0;
    let mut customers = CustomerMap::new(5);
    while s != "}" {
        stdin().read_line(&mut s).expect("wot?");
        for line in s.split(";").map(|line| line.trim()) {
            let parts: Vec<&str> = s.split(",").map(|part| part.trim()).collect();
            match parts[0] {
                "pass" => {
                    t += 1;
                },
                "buy" => {
                    if let Ok(code) = usize::from_str_radix(parts[1], 10) {
                        let mut customer = Customer {
                            code,
                            vehicles: (0..vehicles.len).map(|_| 0).collect(),
                        };
                        t += 5;
                        for buy_request in parts.iter().skip(2) {
                            let buy_request: Vec<&str> = buy_request.split(":").collect();
                            let mut vehicle = vehicles.get(buy_request[0].to_string()).expect("No such vehicle");
                            let ticket_count = usize::from_str_radix(buy_request[1], 10).expect("Ticket count is not a decimal number");
                            t += ticket_count;
                            customer.vehicles[vehicle.number] += ticket_count;
                            vehicle.add_customer(&customer);
                        }
                        customers.add(code, &mut customer);
                    } else {
                        panic!("Invalid national code. it must be a positive integer");
                    }
                },
                "play" => {
                    let code = usize::from_str_radix(parts[1], 10).expect("Invalid national code. It must be a decimal positive integer");
                    let vehicle = vehicles.get(parts[2].to_string()).expect("No such vehicle");
                    let plays_number = usize::from_str_radix(parts[3], 10).expect("Invalid play number. It must be a decimal positive integer");
                    if let Some(customer) = customers.get(code) {
                        if customer.vehicles[vehicle.number] < plays_number {
                            println!("This customer doesn't have enough tickets");
                        } else {
                            customer.vehicles[vehicle.number] -= plays_number;
                            if let Ok(elasped) = vehicle.run() {
                                t += elasped;
                            } else {
                                println!("No enough customer onboard, yet");
                            }
                        }
                    } else {
                        panic!("No such customer");
                    }
                },
            }
        }
        
    }

    println!("h {}", s);
}
