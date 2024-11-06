use core::num;
use std::str::{Bytes, Chars};

use crate::katas::solution::Solution;
use std::collections::HashMap;

impl Solution {
    ///     You are given a 0-indexed array of positive integers nums.
    /// In one operation, you can swap any two adjacent elements if they have the same number of
    /// set bits
    /// . You are allowed to do this operation any number of times (including zero).
    /// Return true if you can sort the array, else return false.
        pub fn can_sort_array(nums: Vec<i32>) -> bool {
           // Check if the array is already sorted
        if nums.windows(2).all(|w| w[0] <= w[1]) {
            return true;
        }

        let mut groups: std::collections::HashMap<u32, Vec<i32>> = std::collections::HashMap::new();

        // Group numbers by the count of 1's in their binary representation
        for &num in &nums {
            let count_ones = num.count_ones();
            groups.entry(count_ones).or_insert(Vec::new()).push(num);
        }

        // Sort each group individually
        for group in groups.values_mut() {
            group.sort();
        }

        // Merge the sorted groups back into the nums array
        let mut sorted_nums: Vec<i32> = Vec::with_capacity(nums.len());
        for count_ones in 0..=32 {
            if let Some(group) = groups.get(&count_ones) {
                sorted_nums.extend(group);
            }
        }
        println!("sorted_nums {:?}", sorted_nums);
        // Check if the array is sorted
        sorted_nums.windows(2).all(|w| w[0] <= w[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_sort_array_1() {
        assert_eq!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]), true)
    }
    #[test]
    fn test_can_sort_array_2() {
        assert_eq!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]), true)
    }
    
    #[test]
    fn test_can_sort_array_3() {
        assert_eq!(Solution::can_sort_array(vec![3, 16, 8, 4, 2]), false)
    }

    #[test]
    fn test_can_sort_array_4() {
        assert_eq!(Solution::can_sort_array(vec![20, 16]), false)
    }
    #[test]
    fn test_can_sort_array_5() {
        assert_eq!(Solution::can_sort_array(vec![136,256,10]), false)
    }

}
