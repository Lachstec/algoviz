use rand::thread_rng;
use rand::prelude::SliceRandom;

#[derive(Debug, Copy, Clone)]
pub enum ArrayOrder {
    Ordered,
    Reversed,
    Unordered,
}

#[derive(Debug)]
pub struct Array {
   data: Vec<u32>,
}

impl Array {

    pub fn new(amount: usize, min: u32, max: u32, order: ArrayOrder) -> Self {
        let arr: Vec<u32> = match order {
            ArrayOrder::Ordered => (1..=amount).map(|val| val as u32).collect(),
            ArrayOrder::Reversed => (1..=amount).rev().map(|val| val as u32).collect(),
            ArrayOrder::Unordered => {
                let mut arr: Vec<u32> = (1..=amount).map(|val| val as u32).collect();
                arr.shuffle(&mut thread_rng());
                arr
            },
        };

        Self{ data: arr }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn value_at(&self, index: usize) -> Option<&u32> {
       self.data.get(index) 
    }

    pub fn swap(&mut self, first: usize, second: usize) {
        self.data.swap(first, second);
    }
}
