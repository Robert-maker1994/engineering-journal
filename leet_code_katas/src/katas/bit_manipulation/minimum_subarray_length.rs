//! Shortest Subarray with OR at least k 2
//! You are given an array nums of non-negative integers and an integer of k. 
//! An array is called special if the bitwise or of all its elements is at least k
//! return the length of the shortest special non empty subarray of nums or return -1 if no special subarrays exist. 


use super::BitManipulation;

/// Finds the minimum length of a subarray such that the bitwise OR of all elements in the subarray is at least `k`.
///
/// # Arguments
///
/// * `nums` - A vector of integers representing the array.
/// * `k` - An integer representing the target bitwise OR value.
///
/// # Returns
///
/// Returns the length of the smallest subarray with a bitwise OR of at least `k`. 
/// If no such subarray exists, returns `-1`.
///
/// # Approach
///
/// This function uses the sliding window approach to find the minimum subarray length:
///
/// 1. Initialize `left` pointer to the start of the array and `current_or` to 0.
/// 2. Iterate over the array with the `right` pointer.
/// 3. For each element at `right`, include it in the current subarray by performing a bitwise OR with `current_or`.
/// 4. While `current_or` is greater than or equal to `k`, update the minimum length and try to shrink the window from the left by performing a bitwise AND with the negation of the element at `left` and incrementing `left`.
/// 5. If no valid subarray is found, return `-1`. Otherwise, return the minimum length found.
impl BitManipulation {
    
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut min_len = n as i32 + 1;
        let mut left = 0;
        let mut current_or = 0;

        for right in 0..n {
            current_or |= nums[right];
            while current_or >= k {
            min_len = min_len.min((right - left + 1) as i32);
            current_or &= !nums[left];
            left += 1;
            }
        }

        if min_len == n as i32 + 1 {
            -1
        } else {
            min_len
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::katas::bit_manipulation::{self, BitManipulation};
    
    #[test]
    fn test_minimum_subarray_length() {
        assert_eq!(BitManipulation::minimum_subarray_length(vec![1,2,3], 2), 1);
    }

    #[test]
    fn test_minimum_subarray_length2() {
        assert_eq!(BitManipulation::minimum_subarray_length(vec![2,1,8], 10), 3);
    }

    #[test]
    fn test_minimum_subarray_length3() {
        assert_eq!(BitManipulation::minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
