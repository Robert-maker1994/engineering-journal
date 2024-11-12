use super::Arrays;

impl Arrays {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    	use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items.clone();
        let mut ans = vec![0; queries.len()];

        // Sort items by price
        items.sort_by_key(|item| item[0]);

        // Update max beauty so that each price has the maximum beauty seen up to that price
        let mut max_beauty = items[0][1];
        for i in 0..items.len() {
            max_beauty = max(max_beauty, items[i][1]);
            items[i][1] = max_beauty;
        }

        // Process each query using binary search
        for (i, &query) in queries.iter().enumerate() {
            ans[i] = Solution::binary_search(&items, query);
        }

        ans
    }

    fn binary_search(items: &[Vec<i32>], target_price: i32) -> i32 {
        let mut left = 0;
        let mut right = items.len() as i32 - 1;
        let mut max_beauty = 0;

        while left <= right {
            let mid = (left + right) / 2;
            if items[mid as usize][0] > target_price {
                right = mid - 1;
            } else {
                // Found viable price; update max beauty and move right
                max_beauty = max(max_beauty, items[mid as usize][1]);
                left = mid + 1;
            }
        }

        max_beauty
    }
}

