### Understanding the heap and the Stack

The heap and the stack are two fundamental memory allocation mechanisms used in programming languages. Understanding their differences and how they work is crucial for efficient memory management and program performance.

## The Stack 
The stack operates in a first-in, first-out (FIFO) flow.  It is used to store local variables, functions parameters and return addresses. The stack is and efficient but limited size.

# Characteristics of the Stack
- Fast Allocation and Deallocation: Memory allocation and deallocation on the stack are very fast because they follow a simple LIFO order.
- Limited Size: The stack has a limited size, which can lead to stack overflow if too much data is stored.
- Automatic Management: In many programming languages, the stack is automatically managed, ensuring that variables are deallocated when they go out of scope.

```
void function() {
    int x = 42; // x is stored on the stack
    float y = 3.14; // y is also stored on the stack
    // When the function exits, x and y are automatically deallocated
}
```
 
## The Heap 

The heap is a region of memory used for dynamic memory allocation. It is used for storing data that needs to live beyond the scope of a single function or for data that is too large to fit on the stack.

# Characteristics of the Heap

- Dynamic Allocation: Memory on the heap is allocated dynamically at runtime using functions like malloc in C, new in C++, or new in Java.
- Slower Allocation and Deallocation: Allocating and deallocating memory on the heap is slower than on the stack because it involves more complex bookkeeping.
- Manual Management: In many programming languages, heap memory must be manually managed, meaning the programmer is responsible for deallocating memory when it is no longer needed.

```
void function() {
    int* x = (int*)malloc(sizeof(int)); // x is a pointer to a value stored on the heap
    *x = 42;
    float* y = (float*)malloc(sizeof(float)); // y is a pointer to a value stored on the heap
    *y = 3.14;
    // When the function exits, x and y must be manually deallocated
    free(x);
    free(y);
}
```
## When to Use the Heap and the Stack
# Use the Stack:
- For small, short-lived variables.
- For function parameters and local variables.
- When you need fast allocation and deallocation.

# Use the Heap:

- For large data structures that need to live beyond the scope of a single function.
- For data that needs to be shared or modified across different parts of the program.
- When you need dynamic memory allocation.

## Conclusion 
The stack is fast and automatically managed, making it ideal for small, short-lived variables. The heap is used for dynamic memory allocation, allowing for larger and more complex data structures. By leveraging the strengths of both the heap and the stack you shall be able to manage and improve performance and reliability of your programs 


# Resources 
- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html