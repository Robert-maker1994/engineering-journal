use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }

        let mut total_sum: i64 = 0;
        for &num in &nums {
            total_sum += num as i64;
        }

        let rem = total_sum as i32 % p;
        if rem == 0 {
            return 0;
        }

        let mut prefix_mod = HashMap::new();
        prefix_mod.insert(0, -1);
        let mut prefix_sum: i64 = 0;
        let mut min_length = len as i32;

        for (i, &num) in nums.iter().enumerate() {
            prefix_sum += num as i64;
            let current_mod = prefix_sum as i32 % p;
            let target_mod = (current_mod - rem + p) % p;

            if let Some(&index) = prefix_mod.get(&target_mod) {
                min_length = min_length.min(i as i32 - index);
            }

            prefix_mod.insert(current_mod, i as i32);
        }

        if min_length == len as i32 {
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
        let res = 1;
        // spell-checker: disable-next-line
        assert_eq!(Solution::min_subarray(nums, p), res);
    }

    #[test]
    fn test_2() {
        let nums = vec![6, 3, 5, 2];
        let p = 9;
        let res = 2;
        assert_eq!(Solution::min_subarray(nums, p), res);
    }
    
    #[test]
    fn test_4() {
        let nums = vec![1000000000,1000000000,1000000000];
        let p = 3;
        let res = 0;
        assert_eq!(Solution::min_subarray(nums, p), res);
    }
}