struct MyCircularDeque {
    q: Vec<i32>,
}

impl MyCircularDeque {

     fn new(k: i32) -> Self {
        MyCircularDeque {
            q: Vec::with_capacity(k as usize),
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        // check if the deque is full
        if self.is_full() {
            return false;
        }
        // insert the value at the front
        self.q.insert(0, value);
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        // check if the deque is full
        if self.is_full() {
            return false;
        }
        // insert the value at the end
        self.q.push(value);
        true
    }

    fn delete_front(&mut self) -> bool {
        // check if the deque is empty
        if self.is_empty() {
            return false;
        }

        self.q.remove(0);
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.q.pop();
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.q[0]
    }
    
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.q[self.q.len() - 1]
    }

    fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    fn is_full(&self) -> bool {
        self.q.len() == self.q.capacity()
    }
}


