/// Structure to represent a weighted edge in the graph
struct Edge {
    source: usize,
    destination: usize,
    weight: i32,
}

/// Structure to represent the graph
struct Graph {
    vertices: usize,
    edges: Vec<Edge>,
}

impl Graph {
    /// Create a new graph with the given number of vertices
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            edges: Vec::new(),
        }
    }

    /// Add a new edge to the graph
    fn add_edge(&mut self, source: usize, destination: usize, weight: i32) {
        self.edges.push(Edge {
            source,
            destination,
            weight,
        });
    }

    /// Run the Bellman-Ford algorithm to find the shortest path from the source vertex to all other vertices
    fn bellman_ford(&self, source: usize) -> Vec<Option<i32>> {
        let mut distance = vec![None; self.vertices];
        distance[source] = Some(0);

        // Relax edges repeatedly
        for _ in 0..self.vertices - 1 {
            for edge in &self.edges {
                if let Some(dist) = distance[edge.source] {
                    if distance[edge.destination].is_none() || dist + edge.weight < distance[edge.destination].unwrap() {
                        distance[edge.destination] = Some(dist + edge.weight);
                    }
                }
            }
        } 

        // Check for negative-weight cycles
        for edge in &self.edges {
            if let Some(dist) = distance[edge.source] {
                if distance[edge.destination].is_none() || dist + edge.weight < distance[edge.destination].unwrap() {
                    println!("Negative cycle detected");
                    return vec![None; self.vertices];
                }
            }
        }


        distance
    }
}

fn main() {
    let mut graph = Graph::new(9);
    graph.add_edge(0, 1, -5);
    graph.add_edge(0, 2, -4);
    graph.add_edge(1, 3, -3);
    graph.add_edge(1, 2, -2);
    graph.add_edge(2, 1, -1);
    graph.add_edge(2, 3, -2);
    graph.add_edge(2, 4, -5);
    graph.add_edge(3, 2, -3);
    graph.add_edge(3, 1, -3);
    graph.add_edge(4, 3, -2);
    graph.add_edge(8, 0, -5);


    let distances = graph.bellman_ford(0);
    println!("Shortest distances from vertex 0:");
    for (i, dist) in distances.into_iter().enumerate() {
        match dist {
            Some(dist) => println!("Vertex {}: {}", i, dist),
            None => println!("Vertex {}: No path", i),
        }
    }
}