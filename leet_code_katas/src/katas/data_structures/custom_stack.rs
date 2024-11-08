
pub struct _CustomStack {
    _s: Vec<i32>,
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
impl _CustomStack {
fn _new(capacity: i32) -> Self {
        _CustomStack {
            _s: Vec::with_capacity(capacity.try_into().unwrap()),
        }
    }

    fn _push(&mut self, x: i32) {
        if self._s.len() < self._s.capacity() {
            self._s.push(x);
        }
    }

    fn _pop(&mut self) -> i32 {
        if self._s.is_empty() {
            return -1;
        }

        self._s.pop().unwrap()
    }

    fn _increment(&mut self, k: i32, val: i32) {
        let limit = k.min(self._s.len() as i32) as usize;
        for i in 0..limit {
            self._s[i] += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_stack() {
        let mut stack = _CustomStack::_new(3);
        stack._push(1);
        stack._push(2);
        assert_eq!(stack._pop(), 2);
        assert_eq!(stack._pop(), 1);
        assert_eq!(stack._pop(), -1);
    }

    #[test]
    fn test_increment() {
        let mut stack = _CustomStack::_new(5);
        stack._push(1);
        stack._push(2);
        stack._push(3);
        stack._increment(3, 5);
        println!("test {:?}", stack._s);
        assert_eq!(stack._pop(), 8);
        assert_eq!(stack._pop(), 7);
        assert_eq!(stack._pop(), 6);
    }

    #[test]
    fn test_push_beyond_capacity() {
        let mut stack = _CustomStack::_new(2);
        stack._push(1);
        stack._push(2);
        stack._push(3); // This push should be ignored as capacity is 2
        assert_eq!(stack._pop(), 2);
        assert_eq!(stack._pop(), 1);
        assert_eq!(stack._pop(), -1);
    }

    #[test]
    fn test_increment_beyond_length() {
        let mut stack = _CustomStack::_new(3);
        stack._push(1);
        stack._push(2);
        stack._increment(5, 10); // Increment more elements than present
        assert_eq!(stack._pop(), 12);
        assert_eq!(stack._pop(), 11);
        assert_eq!(stack._pop(), -1);
    }
}
