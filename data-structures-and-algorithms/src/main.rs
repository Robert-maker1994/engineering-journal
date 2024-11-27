use std::collections::HashSet;

fn find_champion(n: i32, edges: Vec<Vec<usize>>) -> i32 {
    let mut s: HashSet<i32> = (0..n).collect();
    for e in edges { 
        s.remove(&(e[1] as i32)); 
    }
    if s.len() == 1 { *s.iter().next().unwrap() } else { -1 }
}

fn main() {
    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
    let champion = find_champion(n, edges);
    println!("The champion is: {}", champion);
}