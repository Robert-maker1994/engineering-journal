use std::collections::HashMap;

pub fn valid_path(n: i32, edges:  Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    // Initialization stage 
    let mut visited_nodes: Vec<bool> = vec![false; n as usize];
    
    // Create the graph as an adjacency list
    let mut graph: HashMap<usize, Vec<i32>> = HashMap::new();
    for (i, e) in edges.into_iter().enumerate() {
        graph.entry(i).or_insert(Vec::new()).push(e[0]);
        graph.entry(i).or_insert(Vec::new()).push(e[1]);
    }

    // Use a stack for DFS
    let mut stack = vec![source];

    while let Some(node) = stack.pop() {
        if node == destination {
            return true
        } 
        if !visited_nodes[node as usize] {
            visited_nodes[node as usize] = true;
            if let Some(neighbors) = graph.get(&(node as usize)) {
                    for &neighbor in neighbors {
                        if !visited_nodes[neighbor as usize] {
                                    stack.push(neighbor);
                                }
                        }
                        }
                    }
                }

    println!("stack {:?}", stack);


    false
}

fn main() {
  

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_path() {
        assert_eq!(valid_path(3, vec![vec![0,1],vec![1,2],vec![2,0]], 0, 2), true);
    }

    #[test]
    fn test_find_path1() {
        assert_eq!(valid_path(6, vec![vec![0,1],vec![0,2],vec![3,5],vec![5,4],vec![4,3]], 0, 5), false);
    }
}