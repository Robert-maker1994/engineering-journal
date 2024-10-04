use std::collections::HashMap;

impl Solution {
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
        assert_eq!(Solution::min_subarray(nums, p), 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![6, 3, 5, 2];
        let p = 9;
        assert_eq!(Solution::min_subarray(nums, p), 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3];
        let p = 3;
        assert_eq!(Solution::min_subarray(nums, p), 0);
    }
}