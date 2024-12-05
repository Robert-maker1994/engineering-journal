use core::result::Result::Ok;

struct Graph {
    vertex: Vec<i32>,        // List of vertices
    edges: Vec<Vec<i32>>,    // List of edges, each edge is a vector of vertex indices
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
        
        // Initialize the adjacency matrix with zeros
        let mut matrix = vec![vec![0; n]; n];

        // Fill the matrix with edges
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

        // Display the matrix
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
    
    // Adding edges as a list of vertex indices
    graph.add_edge(vec![0, 1, 2]);
    graph.add_edge(vec![0, 3]);
    graph.add_edge(vec![1, 3]);

    // Display the adjacency matrix
    graph.display_matrix();
}
