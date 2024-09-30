struct MyCircularDeque {
    q: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

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

//     let mut obj = MyCircularDeque::new(5);
//     let ret_1: bool = obj.insert_front(2);
//     let ret_2: bool = obj.insert_last(4);
//     let ret_3: bool = obj.delete_front();
//     let ret_4: bool = obj.delete_last();
//     let ret_5: i32 = obj.get_front();
//     let ret_6: i32 = obj.get_rear();
//     let ret_7: bool = obj.is_empty();
//     let ret_8: bool = obj.is_full();

