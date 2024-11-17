//! Kata Link https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/editorial/ 
//! You are given an array of integers nums of length n and a positive integer k.
//! The power of an array is defined as:
//! - Its maximum element if all of its elements are consecutive and sorted in ascending order.
//! - -1 otherwise.
//! You need to find the power of all subarrays of nums of size k.
//! Return an integer array results of size n - k + 1, where results[i] is the power of nums[i..(i + k - 1)].

use super::Arrays;

impl Arrays {
    /// Solution I made a brute force where I separated into sub arrays and did a check whether the next number was a consective and sorted order. If not I broke the for range and returned the number or returned -1 
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len = nums.len();
        let mut results: Vec<i32> = vec![];

        for start in 0..=len {
            let mut c_and_s = true;

            let i = start + k as usize - 1;
          
            if i < len {
                for sub in start..i {
                    if nums[sub + 1] != nums[sub] + 1 {
                        c_and_s = false;
                        break;
                    }
                }

                if c_and_s {
                    results.push(nums[start + k as usize - 1]);
                } else {
                    results.push(-1);
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results_array() {
        assert_eq!(
            Arrays::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            vec![3, 4, -1, -1, -1]
        )
    }
}
