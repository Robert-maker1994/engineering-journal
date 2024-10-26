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