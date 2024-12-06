# Dijkstra’s Algorithm

##

```rust 
# 
#use std::collections::{BinaryHeap, HashMap, HashSet};
#use std::cmp::Ordering;
#use std::usize;

#struct Graph {
#    edges: HashMap<usize, Vec<(usize, u32)>>, // Node -> (Neighbor, Weight)
#}
#
#impl Graph {
#    fn new() -> Self {
#        Graph {
#            edges: HashMap::new(),
#        }
#    }
#
#    fn add_edge(&mut self, from: usize, to: usize, weight: u32) {
#        self.edges.entry(from).or_insert_with(Vec::new).push((to, weight));
#        // self.edges.entry(to).or_insert_with(Vec::new).push((from, weight)); // For undirected graph
#    }
#
#    fn print(self) {
#        let l = self.edges.len();
#        for (key, value) in self.edges.iter() {
#            println!("[{}]: v {:?} ", key, value);
#
#        }
#    }
#}

#[derive(Eq, PartialEq)]
#struct Node {
#    distance: u32,
#    index: usize,
#}
#
#impl Ord for Node {
#    fn cmp(&self, other: &Self) -> Ordering {
#        other.distance.cmp(&self.distance) // Reverse the order for min-heap
#    }
#}
#
#impl PartialOrd for Node {
#    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
#        Some(self.cmp(other))
#    }
#}
# 
fn dijkstra(graph: &Graph, start: usize) -> HashMap<usize, u32> {
    let mut dist = HashMap::new();
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    for &node in graph.edges.keys() {
        dist.insert(node, usize::MAX as u32); // infinity
    }

    dist.insert(start, 0);

    pq.push(Node {
        distance: 0,
        index: start
    });

    while let Some(Node { distance, index }) = pq.pop() {
        if visited.contains(&index) {
            continue;
        }

        visited.insert(index);
             // Explore the neighbors
             if let Some(neighbors) = graph.edges.get(&index) {
                for (neighbor, weight) in neighbors {
                    let new_dist = distance + weight;
                    if new_dist < *dist.get(neighbor).unwrap_or(&(usize::MAX as u32)) {
                        dist.insert(*neighbor, new_dist);
                        pq.push(Node {
                            distance: new_dist,
                            index: *neighbor,
                        });
                    }
                }
            }
        }
    dist
}

fn main() {
    let mut graph = Graph::new();

    graph.add_edge(0, 1, 1);
    graph.add_edge(0, 2, 4);
    graph.add_edge(1, 2, 2);
    graph.add_edge(1, 3, 5);
    graph.add_edge(2, 3, 1);

    let start_node = 0;
    let distances = dijkstra(&graph, start_node);

    // Output the shortest distances from the source node
    for (node, dist) in distances {
        println!("Distance from {} to {}: {}", start_node, node, dist);
    }
}

```