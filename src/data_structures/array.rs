pub struct Array {
    pub data: Vec<i32>,
}

impl Array {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn set(&mut self, index: usize, value: i32) {
        self.data[index] = value;
    }

    pub fn get(&self, index: usize) -> i32 {
        self.data[index]
    }
}

// TODO: Implement methods for other array operations (e.g., insert, delete).
// TODO: Add tests to verify the correctness of the array operations.