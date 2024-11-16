
# What is a Stack?
A stack is a fundamental data structure in computer science that operates on a **Last In, First Out** (LIFO) principle. This means that the last element added to the stack will be the first one to be removed. Think of it like a stack of plates: you can only take the top plate off the stack, and you can only add a new plate on top.

## Key Operations
- Push: Add an element to the top of the stack.
- Pop: Remove the top element from the stack.
- Peek/Top: View the top element without removing it.
- IsEmpty: Check if the stack is empty.

```
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop() // Removes and returns the last item
    }

    fn peek(&self) -> Option<&T> {
        self.items.last() // Returns a reference to the last item
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}
´´´