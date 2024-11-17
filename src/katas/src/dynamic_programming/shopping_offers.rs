use super::DynamicProgramming;

impl DynamicProgramming {
/// This function calculates the minimum cost to satisfy the shopping needs given the price of each item,
/// special offers, and the required quantities of each item.
///
/// # Arguments
///
/// * `price` - A vector of integers where each element represents the price of an item.
/// * `special` - A vector of vectors where each sub-vector represents a special offer. The first `n` elements
///   of each sub-vector represent the quantities of each item in the offer, and the last element represents
///   the price of the offer.
/// * `needs` - A vector of integers where each element represents the required quantity of each item.
///
/// # Returns
///
/// * An integer representing the minimum cost to satisfy the shopping needs.
///

///
/// # Explanation
///
/// The function uses a depth-first search (DFS) algorithm to explore all possible ways to satisfy the shopping needs.
/// It starts by calculating the cost without any special offers. Then, it iterates through each special offer and checks
/// if the offer can be applied (i.e., if the required quantities are greater than or equal to the quantities in the offer).
/// If the offer can be applied, it recursively calculates the cost with the offer applied and updates the minimum cost.
/// Finally, it returns the minimum cost.
///
/// The `dfs` function is a helper function that performs the DFS. It takes the price vector, the special offers vector,
/// a mutable reference to the needs vector, and the current position in the special offers vector as arguments. It returns
/// the minimum cost to satisfy the shopping needs from the current position onwards.
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
