# What are Hash Tables? 

Is a collection of key-value pairs known as an entries. HashTable<int, string>. It is designed to provide a 0(1) average-time complexity for basic operations like insertion, deletion and look up. They can also be called dictionaries in some languages. 


## Pros and Cons of Hash Tables
Pros:
    - Fast Operations: Average-case time complexity for insertion, deletion, and lookup is O(1).
    - Flexible Keys: Can store and retrieve values using complex keys like strings.
Cons:
    - Collisions: Dealing with collisions can complicate implementation.
    - Space Usage: May require more memory than other data structures.
    - Worst-case Performance: Degrades to O(n) when the hash function is poorly designed or the table is too full.


## How a Hash Table Works

Insert:
- Compute the hash of the key.
- Place the key-value pair at the appropriate index (bucket).
- If there's a collision, handle it using chaining or open addressing.
Search:
- Compute the hash of the key.
- Check the corresponding bucket for the key and return the associated value.
Delete:
- Compute the hash of the key.
- Remove the key-value pair from the appropriate bucket.

```
    let mut hash = HashMap::with_capacity(10);
    
    hash.insert(100, "Spongebob");
    hash.insert(123, "Patrick");
    hash.insert(321, "Sandy");
    hash.insert(555, "Squidward");
    hash.insert(777, "Gary");
```

## Best Practices 
TODO