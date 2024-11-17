/// Prime Subtraction Operation
///! [`LeetCode`]: https://leetcode.com/problems/prime-subtraction-operation/description/
/// You are given a 0-indexed integer array nums of length n.
/// You can perform the following operation as many times as you want:
/// Pick an index i that you haven’t picked before, and pick a prime p strictly less than nums[i], then subtract p from nums[i].
/// Return true if you can make nums a strictly increasing array using the above operation and false otherwise.
/// A strictly increasing array is an array whose each element is strictly greater than its preceding element.

use super::Arrays;

impl Arrays {
    pub fn check_prime(x: i32) -> bool {
        if x <= 1 {
            return false;
        }
        for i in 2..=((x as f64).sqrt() as i32) {
            if x % i == 0 {
                return false;
            }
        }
        true
    }

    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let max_element = *nums.iter().max().unwrap();

        // Store the previousPrime array.
        let mut previous_prime = vec![0; (max_element + 1) as usize];
        for i in 2..=max_element {
            if Self::check_prime(i) {
                previous_prime[i as usize] = i;
            } else {
                previous_prime[i as usize] = previous_prime[(i - 1) as usize];
            }
        }

        for i in 0..nums.len() {
            let bound;
            // In case of first index, we need to find the largest prime less
            // than nums[0].
            if i == 0 {
                bound = nums[0];
            } else {
                // Otherwise, we need to find the largest prime, that makes the
                // current element closest to the previous element.
                bound = nums[i] - nums[i - 1];
            }

            // If the bound is less than or equal to 0, then the array cannot be
            // made strictly increasing.
            if bound <= 0 {
                return false;
            }

            // Find the largest prime less than bound.
            let largest_prime = previous_prime[(bound - 1) as usize];

            // Subtract this value from nums[i].
            nums[i] -= largest_prime;
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_sub_operation() {
        assert_eq!(Arrays::prime_sub_operation(vec![6,8,11,12]), true);
        assert_eq!(Arrays::prime_sub_operation(vec![4,9,6,10]), true);
    }

    

    #[test]
    fn test_prime_sub_operation_1() {
        assert_eq!(Arrays::prime_sub_operation(vec![5, 8, 3]), false);
     }
}
