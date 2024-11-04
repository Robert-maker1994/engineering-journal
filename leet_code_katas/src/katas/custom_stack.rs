
pub struct CustomStack {
    s: Vec<i32>,
}


/// A custom stack implementation with a fixed capacity.
///
/// # Methods
///
/// - `new(capacity: i32) -> Self`:
///   Creates a new `CustomStack` with the specified capacity.
///
/// - `push(&mut self, x: i32)`:
///   Pushes an element `x` onto the stack if the stack is not full.
///
/// - `pop(&mut self) -> i32`:
///   Pops the top element from the stack and returns it. Returns `-1` if the stack is empty.
///
/// - `increment(&mut self, k: i32, val: i32)`:
///   Increments the bottom `k` elements of the stack by `val`. If `k` is greater than the number of elements in the stack, all elements are incremented.
impl CustomStack {
fn new(capacity: i32) -> Self {
        CustomStack {
            s: Vec::with_capacity(capacity.try_into().unwrap()),
        }
    }

    fn push(&mut self, x: i32) {
        if self.s.len() < self.s.capacity() {
            self.s.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        if self.s.is_empty() {
            return -1;
        }

        self.s.pop().unwrap()
    }

    fn increment(&mut self, k: i32, val: i32) {
        let limit = k.min(self.s.len() as i32) as usize;
        for i in 0..limit {
            self.s[i] += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_stack() {
        let mut stack = CustomStack::new(3);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
        assert_eq!(stack.pop(), -1);
    }

    #[test]
    fn test_increment() {
        let mut stack = CustomStack::new(5);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.increment(3, 5);
        println!("test {:?}", stack.s);
        assert_eq!(stack.pop(), 8);
        assert_eq!(stack.pop(), 7);
        assert_eq!(stack.pop(), 6);
    }

    #[test]
    fn test_push_beyond_capacity() {
        let mut stack = CustomStack::new(2);
        stack.push(1);
        stack.push(2);
        stack.push(3); // This push should be ignored as capacity is 2
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
        assert_eq!(stack.pop(), -1);
    }

    #[test]
    fn test_increment_beyond_length() {
        let mut stack = CustomStack::new(3);
        stack.push(1);
        stack.push(2);
        stack.increment(5, 10); // Increment more elements than present
        assert_eq!(stack.pop(), 12);
        assert_eq!(stack.pop(), 11);
        assert_eq!(stack.pop(), -1);
    }
}
