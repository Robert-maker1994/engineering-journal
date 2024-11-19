
///! Find minimum Array End
///! [`LeetCode`]: https://leetcode.com/problems/minimum-array-end/description/?envType=daily-question&envId=2024-11-09
///! Given two integers `n` and ´x´.
///! Construct an array of positive intergers nums of size n where   0 <= i < n - 1, nums[i + 1].
///! The result of the bitwise And operation between all elements of nums X

use super::BitManipulation;

impl BitManipulation {
    /// This function takes two i32s
    /// We first add the value of x into the nums array.
    /// Start the iteration loop at index 1 because we are going to look at the value behind and plus one. 
    /// Ensure that next_value still satisfies the AND 
    /// We want the next_value to have all bits of x
    /// Return the last index of nums convert into i64
    pub fn min_end(mut n: i32,mut x: i32) -> i64 {
        let mut result = 0;
        let mut bit = 1;
        n -= 1;
        while n != 0 || x != 0 {
            let x_bit_is_set = x & 1 == 1;
            let n_bit_is_set = n & 1 == 1;

            if x_bit_is_set || (!x_bit_is_set && n_bit_is_set) {
            result |= bit;
            }

            if !x_bit_is_set {
            n >>= 1;
            }

            x >>= 1;
            bit <<= 1;
        }

        result  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_min_end() {
        assert_eq!(BitManipulation::min_end(6715154, 7193485), 7193485)
    }
}
