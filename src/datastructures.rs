use crate::vehicle::Vehicle;
use crate::customer::Customer;


pub struct CustomerQueue {
    items: Vec<Option<&Customer>>,
    pointer: usize,
}


impl CustomerQueue {
    pub fn new(size: usize) -> Self {
        const NO_ITEM: Option<&Customer> = None;
        CustomerQueue {
            items: (0..size).map(|_| NO_ITEM).collect(),
            pointer: 0,
        }
    }
    pub fn enqueue(&mut self, item: &Customer) -> Result<usize, &str> {
        let pointer = self.pointer;
        if pointer == self.items.len() - 1 {
            return Err("Capacity full");
        }
        self.items[pointer] = Some(item);
        self.pointer += 1;
        Ok(self.items.len() - self.pointer)
    }

    pub fn dequeue(&mut self) -> Result<&Customer, &str> {
        let pointer = self.pointer;
        if let Some(item) = self.items[pointer - 1].take() {
            self.pointer -= 1;
            return Ok(item);
        } else {
            return Err("Queue empty");
        }
    }

    pub fn elements_n(&self) -> usize {
        return self.pointer;
    }

    pub fn empty(&mut self) {
        for i in 0..self.items.len() {
            self.items[i] = None;
        }
    }
}

pub struct VehicleSet {
    vehicles: Vec<Option<Vec<Vehicle>>>,
    vehicles_by_number: Vec<Option<&Vehicle>>,
    pub len: usize,
}

impl VehicleSet {
    pub fn new() -> Self {
        const NOTHING: Option<Vec<Vehicle>> = None;
        const ANOTHER_NOTHING: Option<&Vehicle> = None;
        VehicleSet {
            vehicles: (0..26).map(|_| NOTHING).collect(),
            vehicles_by_number: (0..5).map(|_| ANOTHER_NOTHING).collect(),
            len: 0,
        }
    }

    pub fn add(&mut self, vehicle: Vehicle) -> Result<bool, &str> {
        if let Ok(mut place) = usize::from_str_radix(vehicle.name.split_at(1).0, 36) {
            place -= 10;
            if let Some(cell) = &mut self.vehicles[place] {
                let mut in_there: bool = false;
                for v in cell.iter() {
                    if v.name == vehicle.name {
                        in_there = true;
                    }
                }
                if in_there {
                    return Ok(false);
                } else {
                    cell.push(vehicle);
                    if vehicle.number <= self.vehicles_by_number.len() {
                        if self.vehicles_by_number[vehicle.number].is_some() {
                            return Err("Duplicate number");
                        } else {
                            self.vehicles_by_number[vehicle.number] = Some(&vehicle);
                        }
                    } else {
                        const ANOTHER_NOTHING: Option<&Vehicle> = None;
                        self.vehicles_by_number.append(&mut (self.vehicles_by_number.len()..vehicle.number).map(|_| ANOTHER_NOTHING).collect());
                        self.vehicles_by_number[vehicle.number] = Some(&vehicle);
                    }
                    self.len += 1;
                    return Ok(true);
                }
            } else {
                let mut v: Vec<Vehicle> = vec![];
                v.push(vehicle);
                self.vehicles[place] = Some(v);
                self.len += 1;
                return Ok(true);
            }
        } else {
            return Err("Invalid name");
        }
    }

    pub fn get(&mut self, name: String) -> Result<&Vehicle, &str> {
        if let Ok(mut place) = usize::from_str_radix(name.split_at(1).0, 36) {
            place -= 10;
            if let Some(cell) = &mut self.vehicles[place] {
                for v in cell.iter_mut() {
                    if v.name == name {
                        return Ok(v);
                    }
                }
                return Err("No such vehicle");
            } else { return Err("No such vehicle"); }
        } else {
            return Err("Invalid name");
        }
    }

    pub fn get_by_number(&self, number: usize) -> Result<&Vehicle, &str> {
        match self.vehicles_by_number[number] {
            Some(v) => { return Ok(v); },
            None => { return Err("Invalid number"); }
        }
    }
}

pub struct CustomerMap {
    table1: Vec<Option<&Customer>>,
    table2: Vec<Option<&Customer>>
}

impl CustomerMap {
    pub fn new(size: usize) -> Self {
        const NO_CUSTOMER: Option<&Customer> = None;
        CustomerMap {
            table1: (0..2 << size).map(|_| NO_CUSTOMER).collect(),
            table2: (0..2 << (size - 1)).map(|_| NO_CUSTOMER).collect()
        }
    }
    
    fn hash1(&self, code: usize) -> usize {
        return code % self.table1.len();
    }

    fn hash2(&self, code: usize) -> usize {
        return code % self.table2.len();
    }

    pub fn get(&self, code: usize) -> Option<&Customer> {
        if let Some(customer) = &mut self.table1[self.hash1(code)] {
            return Some(customer);
        } else if let Some(customer) = &mut self.table2[self.hash2(code)] {
            return Some(customer);
        } else {
            return None;
        }
    }

    pub fn add(&self, code: usize, customer: &Customer) -> bool {
        if self.get(code).is_some() { return false; }
        let h1 = self.hash1(code);
        let h2 = self.hash2(code);
        loop {
            if self.table1[h1].is_none() {
                self.table1[h1] = Some(customer);
                return true;
            }
            let customer = {
                let t = self.table1[h1];
                self.table1[h1] = Some(customer);
                t
            };
            if self.table2[h2].is_none() {
                self.table2[h2] = customer;
                return true;
            }
            let customer = {
                let t = self.table2[h2];
                self.table2[h2] = customer;
                t
            };
        }
    }
}
