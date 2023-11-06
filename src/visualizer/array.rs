#[derive(Debug)]
pub struct Array {
   data: Vec<u32>,
}

impl Array {
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
