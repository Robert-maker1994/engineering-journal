use std::collections::HashMap;
use super::HashTable;

impl HashTable {
    /// Finds the minimum length of a contiguous subarray that, when removed, 
    /// makes the sum of the remaining elements divisible by `p`.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers representing the array.
    /// * `p` - An integer by which the sum of the remaining elements should be divisible.
    ///
    /// # Returns
    ///
    /// Returns the length of the smallest subarray that can be removed to achieve the goal.
    /// If no such subarray exists, returns -1.
    ///
    /// # Explanation
    ///
    /// The function first calculates the total sum of the array and finds the remainder 
    /// when this sum is divided by `p`. If the remainder is 0, the entire array sum is 
    /// already divisible by `p`, so the function returns 0.
    ///
    /// It then uses a HashMap to keep track of the prefix sums modulo `p` and their indices.
    /// As it iterates through the array, it calculates the current prefix sum and its 
    /// modulo `p`. It then determines the target modulo value that would make the 
    /// remaining sum divisible by `p` if the current subarray is removed.
    ///
    /// If this target modulo value has been seen before, it updates the minimum length 
    /// of the subarray that can be removed. Finally, it returns the minimum length found, 
    /// or -1 if no such subarray exists.
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let total_sum: i64 = nums.iter().map(|&num| num as i64).sum();
        let rem = total_sum % p as i64;
        if rem == 0 {
            return 0;
        }

        let mut prefix_mod = HashMap::new();
        prefix_mod.insert(0, -1);
        let mut prefix_sum: i64 = 0;
        let mut min_length = nums.len() as i32;

        for (i, &num) in nums.iter().enumerate() {
            prefix_sum += num as i64;
            let current_mod = prefix_sum % p as i64;
            let target_mod = (current_mod - rem + p as i64) % p as i64;

            if let Some(&index) = prefix_mod.get(&target_mod) {
                min_length = min_length.min(i as i32 - index);
            }

            prefix_mod.insert(current_mod, i as i32);
        }

        if min_length == nums.len() as i32 {
            -1
        } else {
            min_length
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 1, 4, 2];
        let p = 6;
        assert_eq!(HashTable::min_subarray(nums, p), 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![6, 3, 5, 2];
        let p = 9;
        assert_eq!(HashTable::min_subarray(nums, p), 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3];
        let p = 3;
        assert_eq!(HashTable::min_subarray(nums, p), 0);
    }
}