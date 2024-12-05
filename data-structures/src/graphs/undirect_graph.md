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

- Here is a graph in an adjacency Matrix. 

```rust 

// Graph representation 
struct Graph {
    vertex: Vec<i32>,        
    edges: Vec<Vec<i32>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertex: vec![],
            edges: vec![],
        }
    }

    fn add_vertex(&mut self, v: i32) -> Result<(), &str> {
        self.vertex.push(v);
        Ok(())
    }

    fn add_edge(&mut self, e: Vec<i32>) {
        self.edges.push(e);    
    }

    // Method to create and display the adjacency matrix
    fn display_matrix(&self) {
        let n = self.vertex.len();
        
        let mut matrix = vec![vec![0; n]; n];

        for edge in &self.edges {
            for i in 0..edge.len() {
                for j in i + 1..edge.len() {
                    let u = edge[i] as usize;
                    let v = edge[j] as usize;
                    matrix[u][v] = 1;
                   matrix[v][u] = 1; // Remove for undirected graph
                }
            }
        }

        println!("Adjacency Matrix:");
        for row in matrix.iter() {
            println!("{:?}", row);
        }
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.add_vertex(0).err();
    graph.add_vertex(1).err();
    graph.add_vertex(2).err();
    graph.add_vertex(3).err();    

    graph.add_edge(vec![0, 1, 2]);
    graph.add_edge(vec![0, 3]);
    graph.add_edge(vec![1, 3]);

    graph.display_matrix();
}

```