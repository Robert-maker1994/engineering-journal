

# What is a Queue?

A *queue* is a linear data structure, it follows the **First In, First Out** (FIFO) principle.This means the first item added to the queue is the first one to be removed, much like a line of people waiting for a service.

## What is a linear data structure? 

Linear means a straight line or one dimension. 

A *linear data structure* is a type of data structure where elements are arranged sequentially, one after another, in a linear order. Each element is connected to it's previous and next element. (except the first and last elements). 

Characteristics of a linear data structure? 
- Sequential Arrangement
- Single Level
- Traversal
- Contiguity or Linkage

## Key characteristics of queue?
- FIFO Order: The order of insertion is the order of removal.
- Two Ends:
    - Front: Where elements are dequeued (removed).
    - Rear: Where elements are enqueued (added).
- Dynamic Size: Can grow or shrink as elements are added or removed (depending on implementation).

## Types of Queues?

- *Simple Queue*: Follows the basic FIFO principle.
- [*Circular Queue*](./cicular_queue.md): The rear wraps around to the front when the end of the queue is reached, optimizing space.
- *Priority Queue*: Each element is associated with a priority, and elements are dequeued based on their priority, not their order.
- Deque (Double-Ended Queue): Allows insertion and deletion from both ends.

## Where would you use a Queue?

1. Scheduling:
    - CPU task scheduling.
    - Printer task scheduling.
2. Data Streaming:
    - Handling asynchronous data.
3. Breadth-First Search (BFS):
    - A graph traversal algorithm uses a queue.
4. Caching:
    - Maintaining a fixed-size cache using FIFO policy.
5. Order Processing:
    - Managing requests or events in the order they arrive.

## Implementing 
Implementing a queue in javascript would have to implement the queue using an array. For example in other languages are already built into the lang. [VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)

```rust
class Queue {
  constructor() {
    this.items = [];
  }

  enqueue(item) {
    this.items.push(item); // Add to the rear
  }

  dequeue() {
    if (!this.isEmpty()) {
      return this.items.shift(); // Remove from the front
    }
    return "Queue is empty!";
  }

  peek() {
    return this.isEmpty() ? "Queue is empty!" : this.items[0];
  }

  isEmpty() {
    return this.items.length === 0;
  }

  size() {
    return this.items.length;
  }
}
```