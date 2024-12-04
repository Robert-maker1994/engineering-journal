
# Directed Graph

A **directed graph** (or digraph) is a set of nodes connected by edges, where the edges have a direction associated with them. This means that if there is an edge from node A to node B, it does not imply that there is an edge from node B to node A.

## Terminology

- **Vertex (Node)**: A fundamental part of a graph, representing an entity.
- **Edge (Arc)**: A connection between two vertices in a graph, with a direction.
- **In-degree**: The number of edges coming into a vertex.
- **Out-degree**: The number of edges going out of a vertex.


```rust
use std::collections::HashMap;

struct DirectedGraph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl DirectedGraph {
    fn new() -> Self {
        DirectedGraph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: String) {
        self.adjacency_list.entry(vertex).or_insert(Vec::new());
    }

    fn add_edge(&mut self, source: String, destination: String) {
        self.adjacency_list.entry(source.clone()).or_insert(Vec::new()).push(destination.clone());
        self.adjacency_list.entry(destination).or_insert(Vec::new());
    }

    fn print_graph(&self) {
        for (vertex, edges) in &self.adjacency_list {
            println!("{} -> {:?}", vertex, edges);
        }
    }
}

fn main() {
    let mut graph = DirectedGraph::new();
    graph.add_vertex("A".to_string());
    graph.add_vertex("B".to_string());
    graph.add_edge("A".to_string(), "B".to_string());
    graph.add_edge("A".to_string(), "C".to_string());
    graph.add_edge("B".to_string(), "D".to_string());
    graph.add_edge("C".to_string(), "D".to_string());
    graph.add_edge("D".to_string(), "A".to_string());

    graph.print_graph();
}
```

## Algorithms

Common algorithms used with directed graphs include:

- Depth-First Search (DFS) 
- Breadth-First Search (BFS)
- Topological Sorting
- Shortest Path Algorithms (e.g., Dijkstra's, Bellman-Ford)
