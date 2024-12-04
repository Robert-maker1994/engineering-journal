# Undirected Graph

An **undirected graph** is a type of graph in which edges have no direction. This means that the relationship between the nodes is bidirectional. In other words, if there is an edge between node A and node B, you can travel from A to B and from B to A.

## Properties

- **Vertices (Nodes)**: The fundamental units of the graph.
- **Edges (Links)**: The connections between the vertices.
- **Degree of a Vertex**: The number of edges connected to a vertex.


### Example in Rust

Here is an example of how to represent an undirected graph using an adjacency list in Rust:

```rust
use std::collections::HashMap;

struct Graph {
    adjacency_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: i32, v: i32) {
        self.adjacency_list.entry(u).or_insert(Vec::new()).push(v);
        self.adjacency_list.entry(v).or_insert(Vec::new()).push(u);
    }

    fn display(&self) {
        for (node, neighbors) in &self.adjacency_list {
            println!("{}: {:?}", node, neighbors);
        }
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);

    graph.display();
}
```

This code defines a `Graph` struct with an adjacency list represented as a `HashMap`. The `add_edge` method adds an edge between two nodes, and the `display` method prints the adjacency list.
