use std::ops::Index;

 

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        
        fn dfs(price: &Vec<i32>, special: &Vec<Vec<i32>>, needs: &mut Vec<i32>, pos: usize) -> i32 {
            let mut min_cost: i32 = needs.iter().enumerate().map(|(i, &need)| need * price[i]).sum();
            for i in pos..special.len() {
                let mut j = 0;
                while j < needs.len() && needs[j] >= special[i][j] {
                    j += 1;
                }
                if j == needs.len() {
                    for k in 0..needs.len() {
                        needs[k] -= special[i][k];
                    }
                    min_cost = min_cost.min(special[i][needs.len()] + dfs(price, special, needs, i));
                    for k in 0..needs.len() {
                        needs[k] += special[i][k];
                    }
                }
            }
            min_cost
        }
        dfs(&price, &special, &mut needs.clone(), 0)
    }
}

// fn main() {
//    let s = Solution::shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2]);
    
// }
