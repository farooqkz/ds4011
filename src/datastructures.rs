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

