 

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
