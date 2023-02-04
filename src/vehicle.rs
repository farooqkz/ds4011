use crate::datastructures::CustomerQueue;
use crate::customer::Customer;
use crate::consts::*;


pub struct Vehicle {
    pub number: usize,
    pub name: String,
    max_capacity: usize,
    min_capacity: usize,
    pub enabled: bool,
    pub price: usize,
    pub time_per_round: usize,
    pub onboard: Vec<Option<Customer>>,
    pub number_of_onboard_customers: usize,
    queue: CustomerQueue,
}

const NO_CUSTOMER: Option<crate::customer::Customer> = None;

impl Vehicle {
    pub fn new(number: usize, name: String, max_capacity: usize, min_capacity: usize, price: usize, time_per_round: usize) -> Self {
        Vehicle {
            number,
            name,
            max_capacity,
            min_capacity,
            enabled: true,
            price,
            time_per_round,
            onboard: (0..max_capacity).map(|_| None).collect(),
            number_of_onboard_customers: 0,
            queue: CustomerQueue::new(max_capacity*2),
        }
    }
    
    pub fn new_from_string(s: String, number: usize) -> Self {
        let s: Vec<&str> = s.split(",").map(|part| part.trim()).collect();
        let max_capacity = usize::from_str_radix(s[1], 10);
        let min_capacity = usize::from_str_radix(s[2], 10);
        let time_per_round = usize::from_str_radix(s[3], 10);
        let price = usize::from_str_radix(s[4], 10);
        match (max_capacity, min_capacity, time_per_round, price) {
            (Ok(max_capacity), Ok(min_capacity), Ok(time_per_round), Ok(price)) => {
                return Self::new(number, s[0].to_string(), max_capacity, min_capacity, price, time_per_round);
            },
            _ => {
                panic!("Invalid parameters");
            }
        }
    }

    pub fn add_customer(&mut self, customer: Customer) -> Result<usize, &str> {
        if self.queue.elements_n() >= self.max_capacity {
            return Err("No space left on vehicle");
        }
        self.queue.enqueue(customer);
        Ok(self.queue.elements_n())
    }

    pub fn run(&mut self) -> Result<usize, &str> {
        if self.queue.elements_n() < self.min_capacity {
            return Err("No enough customers");
        }
        let t: usize = self.queue.elements_n() * (VEHICLE_GET_OFF_TIME + VEHICLE_GET_ON_TIME) + self.time_per_round;
        self.queue.empty();
        Ok(t)
    }
}
