use std::collections::{HashMap, VecDeque};

fn bfs(start: usize, graph: &HashMap<usize, Vec<usize>>) {
    // Initialization 
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
       
        if let Some(edges) = graph.get(&node) {
            println!("all of the edges for the node {:?}", edges);
            for &neighbor in edges {
                if !visited[neighbor] {
                    // Adds them as visited
                    visited[neighbor] = true;
                    // Adds the neighbors into the queue to find them. 
                    queue.push_back(neighbor);
                }
            }
        }
    }
}

fn main() {
      let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];

    // Initialization stage
    let mut visited_nodes: Vec<bool> = vec![false; edges.len()];

    // Create the graph as an adjacency list
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for edge in edges {
        graph.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
        graph.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
    }

   bfs(0, &graph);
}