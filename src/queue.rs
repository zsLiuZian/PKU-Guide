pub struct Queue {
    older: Vec<usize>,
    newer: Vec<usize>,
}
impl Queue {
    pub fn new() -> Self {
        Self {
            older: Vec::new(),
            newer: Vec::new(),
        }
    }

    pub fn push(&mut self, x: usize) {
        self.newer.push(x);
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.older.is_empty() {
            while let Some(x) = self.newer.pop() {
                self.older.push(x);
            }
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.newer.is_empty()
    }
    
}