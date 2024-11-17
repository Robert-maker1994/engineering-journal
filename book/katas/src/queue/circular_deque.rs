
struct _MyCircularDeque {
    q: Vec<i32>,
}

impl _MyCircularDeque {

    fn _new(k: i32) -> Self {
        _MyCircularDeque {
            q: Vec::with_capacity(k as usize),
        }
    }

    fn _insert_front(&mut self, value: i32) -> bool {
        // check if the deque is full
        if self._is_full() {
            return false;
        }
        // insert the value at the front
        self.q.insert(0, value);
        true
    }

    fn _insert_last(&mut self, value: i32) -> bool {
        // check if the deque is full
        if self._is_full() {
            return false;
        }
        // insert the value at the end
        self.q.push(value);
        true
    }

    fn _delete_front(&mut self) -> bool {
        // check if the deque is empty
        if self._is_empty() {
            return false;
        }

        self.q.remove(0);
        true
    }

    fn _delete_last(&mut self) -> bool {
        if self._is_empty() {
            return false;
        }
        self.q.pop();
        true
    }

    fn _get_front(&self) -> i32 {
        if self._is_empty() {
            return -1;
        }
        self.q[0]
    }
    
    fn _get_rear(&self) -> i32 {
        if self._is_empty() {
            return -1;
        }
        self.q[self.q.len() - 1]
    }

    fn _is_empty(&self) -> bool {
        self.q.is_empty()
    }

    fn _is_full(&self) -> bool {
        self.q.len() == self.q.capacity()
    }
}


