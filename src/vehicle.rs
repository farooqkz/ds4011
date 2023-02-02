use crate::datastructures::Queue;
use crate::customer::Customer;


pub struct Vehicle {
    number: usize,
    name: String,
    max_capacity: usize,
    min_capacity: usize,
    enabled: bool,
    price: usize,
    time_per_round: usize,
    onboard: Vec<Option<Customer>>,
    number_of_onboard_customers: usize,
    queue: Queue<Customer>,
}

const NO_CUSTOMER: Option<crate::customer::Customer> = None;
const MOVE_TIME: usize = 10;

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
            queue: Queue::new(),
        }
    }

    pub fn add_customer(&mut self, customer: crate::customer::Customer) -> Result<usize, &str> {
        if self.queue.elemnents_n() >= self.max_capacity {
            return Err("No space left on vehicle");
        }
        self.queue.enqueue(customer);
        Ok(self.queue.elemnents_n())
    }

    pub fn run(&mut self) -> Result<usize, &str> {
        if self.queue.elemnents_n() < self.min_capacity {
            return Err("No enough customers");
        }
        let t: usize = self.queue.elements_n() * MOVE_TIME * 2;
        self.queue.empty();
        Ok(t)
    }
}
