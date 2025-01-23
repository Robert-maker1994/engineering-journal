# Node.js Event Loop

Node.js is a JavaScript runtime built on Chrome's V8 JavaScript engine. It uses an event-driven, non-blocking I/O model that makes it lightweight and efficient.

## Event Loop

The event loop is what allows Node.js to perform non-blocking I/O operations. Despite JavaScript being single-threaded, the event loop enables Node.js to handle multiple operations concurrently.

### Phases of the Event Loop

1. **Timers**: This phase executes callbacks scheduled by `setTimeout()` and `setInterval()`.
2. **Pending Callbacks**: Executes I/O callbacks deferred to the next loop iteration.
3. **Idle, Prepare**: Internal use only.
4. **Poll**: Retrieves new I/O events; executes I/O related callbacks.
5. **Check**: Executes callbacks scheduled by `setImmediate()`.
6. **Close Callbacks**: Executes close event callbacks, e.g., `socket.on('close', ...)`.

### Example

```javascript
const fs = require('fs');

console.log('Start');

fs.readFile('file.txt', (err, data) => {
    if (err) throw err;
    console.log('File read');
});

setTimeout(() => {
    console.log('Timeout');
}, 0);

console.log('End');
```

Output:
```
Start
End
Timeout
File read
```

In this example, `setTimeout` and `fs.readFile` are non-blocking operations. The event loop handles these operations, allowing the program to continue running other code while waiting for these operations to complete.
