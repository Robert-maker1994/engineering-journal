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