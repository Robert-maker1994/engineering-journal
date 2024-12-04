# Breath First Search 

Is an algorithm to handle graphs and trees, very similar to [DFS](./depth_first_search.md), however, Breath First Search (BFS) approach explores all the neighbors of a node before moving on to the next level. Uses a queue to keep track of nodes explored, looks at the traversal order level by level (why it is used for trees).

## Implementation 

```rust 
use std::collections::{HashMap, VecDeque};


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

    fn bfs(start: usize, graph: &HashMap<usize, Vec<usize>>) {
    
    // Initialization of the bfs
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
}

```

## Limitations 
- Not Suitable for Weighted Graphs
- Memory usages 
- Not the best time complexity. 
