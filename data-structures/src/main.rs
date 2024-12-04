use std::collections::{HashMap, VecDeque};

struct Solution;
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
       
    // Create the graph as an adjacency list
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for edge in 0..n {
        graph.entry(edge[0]).or_insert(Vec::new()).push(edge[1] as usize);
        graph.entry(edge[1] as usize).or_insert(Vec::new()).push(edge[0]as usize);
    }
        bfs(source as usize, &graph, destination as usize)
    }
}
    fn bfs(start: usize, graph: &HashMap<usize, Vec<usize>>, destination: usize) -> bool {
    
    // Initialization of the bfs
    let mut current_path: Vec<usize> = vec![];
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        println!("Node {} {}", node, destination);

        if node == destination {
            return true
        }
        current_path.push(node);
        if let Some(edges) = graph.get(&node) {
            for &neighbor in edges {
                current_path.push(neighbor);
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                    }
                }
            }
        }
        println!("current path {:?}", current_path);

    return false
    }

fn main() {
    // let res = Solution::valid_path(3,vec![vec![0,1],vec![1,2],vec![2,0]], 0, 2);
    // println!("res {}", res);

    // Solution::valid_path(6, vec![vec![0,1],vec![0,2],vec![3,5],vec![5,4],vec![4,3]],  0, 5);

    Solution::valid_path(5, vec![vec![0,4]], 0, 4);
}

