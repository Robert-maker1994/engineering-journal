     /// This function calculates the minimum number of swaps required to group all '1's together in a binary string.
    ///
    /// # Arguments
    ///
    /// * `s` - A binary string consisting of characters '0' and '1'.
    ///
    /// # Returns
    ///
    /// * `i64` - The minimum number of swaps required.
    ///
    /// # Explanation
    ///
    /// The function works by iterating through the string and maintaining counts of '0's and '1's.
    /// It calculates the minimum number of swaps needed to group all '1's together by considering
    /// the number of '0's to the left and the number of '1's to the right at each position.
    ///
    /// # Algorithm
    ///
    /// 1. Convert the input string `s` into a vector of characters.
    /// 2. Count the total number of '1's in the string (`total_black_count`).
    /// 3. Initialize `left_white_count` to 0, which keeps track of the number of '0's encountered so far.
    /// 4. Initialize `right_black_count` to `total_black_count`, which keeps track of the number of '1's remaining to the right.
    /// 5. Initialize `min_swaps` to the maximum possible value of `i32`.
    /// 6. Iterate through each character in the string:
    ///    - If the character is '0', increment `left_white_count`.
    ///    - If the character is '1', decrement `right_black_count`.
    ///    - Update `min_swaps` to the minimum of its current value and the sum of `left_white_count` and `right_black_count`.
    /// 7. Return `min_swaps` as an `i64` value.
    impl Solution {
        pub fn min_swaps(s: String) -> i64 {
            let s: Vec<char> = s.chars().collect();
            
            let total_black_count = s.iter().filter(|&&c| c == '1').count();
            let mut left_white_count = 0;
            let mut right_black_count = total_black_count;
            let mut min_swaps = i32::MAX;
            
            for &c in &s {
                if c == '0' {
                    left_white_count += 1;
                } else {
                    right_black_count -= 1;
                }
                min_swaps = min_swaps.min(left_white_count as i32 + right_black_count as i32);
            }
            
            min_swaps as i64
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_min_swaps() {
            assert_eq!(Solution::min_swaps("101".to_string()), 1);
            assert_eq!(Solution::min_swaps("100".to_string()), 1);
            assert_eq!(Solution::min_swaps("0111".to_string()), 0);
            assert_eq!(Solution::min_swaps("110".to_string()), 1);
            assert_eq!(Solution::min_swaps("111000".to_string()), 3);
            assert_eq!(Solution::min_swaps("010101".to_string()), 3);
            assert_eq!(Solution::min_swaps("000".to_string()), 0);
            assert_eq!(Solution::min_swaps("111".to_string()), 0);
        }
    }
