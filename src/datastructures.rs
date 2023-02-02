use crate::vehicle::Vehicle;
use crate::customer::Customer;

const MAX_QUEUE_CAPACITY: usize = 100;

pub struct Queue<T> {
    items: [Option<T>; MAX_QUEUE_CAPACITY],
    pointer: usize,
}


impl Queue<T> {
    pub fn new<T>() -> Self {
        const NO_ITEM: Option<T> = None;
        Queue {
            items: [NO_ITEM; MAX_QUEUE_CAPACITY],
            pointer: 0,
        }
    }
    pub fn enqueue<T>(&mut self, item: T) -> Result<usize, &str> {
        let pointer = self.pointer;
        if pointer == MAX_QUEUE_CAPACITY - 1 {
            return Err("Capacity full");
        }
        self.items[pointer] = Some(item);
        self.pointer += 1;
        Ok(MAX_QUEUE_CAPACITY - self.pointer)
    }

    pub fn dequeue<T>(&mut self) -> Result<T, &str> {
        let pointer = self.pointer;
        if let Some(item) = self.items[pointer - 1] {
            self.items[pointer - 1] = None;
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
        for i in 0..MAX_QUEUE_CAPACITY {
            self.items[i] = None;
        }
    }
}

pub struct VehicleSet {
    vehicles: Vec<Option<Vec<Vehicle>>>,
}

impl VehicleSet {
    pub fn new() -> Self {
        const NOTHING: Option<Vec<Vehicle>> = None;
        VehicleSet {
            vehicles: (0..26).map(|_| NOTHING).collect(),
        }
    }

    pub fn add(&mut self, vehicle: Vehicle) -> Result<bool, &str> {
        if let Ok(place) = usize::from_str_radix(vehicle.name.split_at(1).0, 36) {
            place -= 10;
            if let Some(cell) = self.vehicles[place] {
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
                    return Ok(true);
                }
            } else {
                let mut v: Vec<Vehicle> = vec![];
                v.push(vehicle);
                self.vehicles[place] = Some(v);
                return Ok(true);
            }
        } else {
            return Err("Invalid name");
        }
    }

    pub fn get(&self, name: String) -> Result<&Vehicle, &str> {
        if let Ok(place) = usize::from_str_radix(name.split_at(1).0, 36) {
            place -= 10;
            if let Some(cell) = self.vehicles[place] {
                for v in cell.iter() {
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
}
