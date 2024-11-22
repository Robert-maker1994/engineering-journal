# Circular Queue 

This data structure uses a fixed-size array in a circular manner to efficiently manage a queue. In a circular queue, the position of the front and rear elements are managed using modulo arithmetic to wrap around the end of the array to the beginning, creating a circular effect.

## Key Concepts
1. Fixed Size: The circular queue has a fixed size, which means it can hold a limited number of elements.

2. Front and Rear Pointers: Two pointers, front and rear, are used to keep track of the positions of the first and last elements in the queue.

3. Circular Behavior: When the rear pointer reaches the end of the array, it wraps around to the beginning of the array if there is space available.

4. Full and Empty Conditions: The queue is considered full when the next position of the rear pointer is the front pointer. The queue is empty when the front and rear pointers are equal.

## What is Modulo Arithmetic

The easiest way to think about modulo arithmetic is a clock, where the numbers wrap around after reaching a certain value. For example, on a 12-hour clock, after 12 comes 1 again. Similarly, in a circular queue, when the rear pointer reaches the end of the array, it wraps around to the beginning using modulo arithmetic. 

So for example, we have an array of integers that represent a 12-clock. [12,1,2,3,4,5,6,7,8,9,10,11] the time now is 10 o'clock (index of 10) and we want to check what times its going to be in 5 hours. 

The new time is going to be - `var new_time = (current_time + 5) % 12;`. Since there are only 12 index in the array we use the modulo to wrap around the index. 


```
pub struct MyCircularQueue {
    data: Vec<Option<i32>>,
    front: i32,
    rear: i32,
    size: i32,
    capacity: i32,
}

impl MyCircularQueue {
    /// Initializes the object with the size of the queue to be k.
    pub fn new(k: i32) -> Self {
        MyCircularQueue {
            data: vec![None; k as usize],
            front: 0,
            rear: 0,
            size: 0,
            capacity: k,
        }
    }

    /// Inserts an element into the circular queue. Return true if the operation is successful.
    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.rear as usize] = Some(value);
        self.rear = (self.rear + 1) % self.capacity;
        self.size += 1;
        true
    }

    /// Deletes an element from the circular queue. Return true if the operation is successful.
    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.data[self.front as usize] = None;
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        true
    }

    /// Gets the front item from the queue. If the queue is empty, return -1.
    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.front as usize].unwrap()
        }
    }

    /// Gets the last item from the queue. If the queue is empty, return -1.
    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let rear_index = if self.rear == 0 {
                self.capacity - 1
            } else {
                self.rear - 1
            };
            self.data[rear_index as usize].unwrap()
        }
    }

    /// Checks whether the circular queue is empty or not.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Checks whether the circular queue is full or not.
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

fn main() {
    let mut queue = MyCircularQueue::new(3);

    assert_eq!(queue.en_queue(1), true);
    assert_eq!(queue.en_queue(2), true);
    assert_eq!(queue.en_queue(3), true);
    assert_eq!(queue.en_queue(4), false); // Queue is full

    assert_eq!(queue.front(), 1);
    assert_eq!(queue.rear(), 3);

    assert_eq!(queue.de_queue(), true);
    assert_eq!(queue.front(), 2);

    assert_eq!(queue.en_queue(4), true); // Enqueue after dequeue
    assert_eq!(queue.rear(), 4);
}
```