/// Transforms the given array into its rank array.
///
/// The rank array is defined such that each element in the original array is replaced by its rank.
/// The rank of an element is its position in the sorted array, starting from 1. If two elements are equal,
/// they receive the same rank.
///
/// # Arguments
///
/// * `arr` - A vector of integers to be transformed into its rank array.
///
/// # Returns
///
/// A vector of integers representing the rank array.
///
/// # Example
///
/// ```
/// let arr = vec![40, 10, 20, 30];
/// let result = Solution::array_rank_transform(arr);
/// assert_eq!(result, vec![4, 1, 2, 3]);
/// ```
///
/// # Panics
///
/// This function will panic if the input vector contains elements that cannot be compared or if the input vector is empty.
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort_unstable();
        let mut rank = 1;
        let mut rank_map = std::collections::HashMap::new();
        for &num in &sorted_arr {
            rank_map.entry(num).or_insert_with(|| {
                let r = rank;
                rank += 1;
                r
            });
        }
        println!("{:?}", rank_map);
        arr.iter().map(|&num| *rank_map.get(&num).unwrap()).collect()
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_rank_transform() {
        assert_eq!(Solution::array_rank_transform(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
        assert_eq!(Solution::array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
        assert_eq!(Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]), vec![5, 3, 4, 2, 8, 6, 7, 1, 3]);
    }
}