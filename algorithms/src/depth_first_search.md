# What is Depth First Search

One good algorithm to help with Graph traversal. Depth First Search (DFS) is nothing too advanced given time and an average person would have come up with this. DFS takes the root node and explores as far as possible along each branch before backtracking. DFS is about searching new nodes/vertex's to help you search a data structure.

## Implementations

1. Recursive implementation
   A recursive function that will be on the call stack until all the vertex have been explored.

```rust
fn main() {

    fn dfs_recursive(node: usize, visited_nodes: &mut Vec<bool>, graph: &HashMap<usize, Vec<usize>>) {
        visited_nodes[node as usize] = true;
    if let Some(edges) = graph.get(&node as &usize) {
        println!("Visited node: {} and the edges {:?}", node, edges);
        for &neighbor in edges {

            if !visited_nodes[neighbor] {
                dfs_recursive(neighbor, visited_nodes, graph);
            }
        }
    }
}

let edges = vec![vec![0,1],vec![1,2],vec![2,0]];

// Initialization stage
let mut visited_nodes: Vec<bool> = vec![false; edges.len() as usize];

// Create the graph as an adjacency list
let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, e) in edges.into_iter().enumerate() {
        graph.entry(i).or_insert(Vec::new()).push(e[0]);
    graph.entry(i).or_insert(Vec::new()).push(e[1]);
}

    dfs_recursive(0, &mut visited_nodes, &graph);

}
```

2. Stack

```rust
use std::collections::HashMap;

fn main() {
let edges = vec![vec![0,1],vec![1,2],vec![2,0]];
  // Initialization stage
  let mut visited_nodes: Vec<bool> = vec![false; edges.len() as usize];

  // Create the graph as an adjacency list
  let mut graph: HashMap<usize, Vec<i32>> = HashMap::new();
  for (i, e) in edges.into_iter().enumerate() {
      graph.entry(i).or_insert(Vec::new()).push(e[0]);
      graph.entry(i).or_insert(Vec::new()).push(e[1]);
  }

  // Use a stack for DFS Starting at the root vertex
  let mut stack = vec![0];

  
  while let Some(node) = stack.pop() {
      
      if !visited_nodes[node as usize] {
          visited_nodes[node as usize] = true;
          
          if let Some(edges) = graph.get(&(node as usize)) {
              println!("The vertex {} and its edges {:?}", node, edges);
              for &neighbor in edges {
                  if !visited_nodes[neighbor as usize] {
                      stack.push(neighbor);
                          }
                      }
                  }
              }
          }
}
```

## Ordering 
1. **Pre-ordering**: A preordering is a list of the vertices in the order that they were first visited by the depth-first search algorithm. The examples above are pre ordering because they check is the vertex has been visited. 

2. **Post-Ordering**: Doing the ordering of a vertices at the end of a path. For example in a tree if you want to delete the nodes that dont have any children. 

```rust
use std::collections::HashMap;

fn dfs_post_order(node: usize, visited_nodes: &mut Vec<bool>, graph: &HashMap<usize, Vec<usize>>, result: &mut Vec<usize>) {
    visited_nodes[node] = true;
    if let Some(edges) = graph.get(&node) {
        for &neighbor in edges {
            if !visited_nodes[neighbor] {
                dfs_post_order(neighbor, visited_nodes, graph, result);
            }
        }
    }
    result.push(node);
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

    // Result vector to store the post-ordering
    let mut result = Vec::new();

    // Perform DFS starting from node 0
    dfs_post_order(0, &mut visited_nodes, &graph, &mut result);

    // Print the post-ordering result
    println!("Post-ordering: {:?}", result);
}
```
