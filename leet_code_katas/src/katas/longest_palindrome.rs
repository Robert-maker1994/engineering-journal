use std::vec;

pub struct Solution;


// A palidormic substring is a substring that reads the same forwards and backwards.
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let len = s.len();
        if len == 0 {
            return "".to_string();
        }

        let mut dp = vec![vec![false; len]; len];
        let mut start = 0;
        let mut max_len = 1;

        for i in 0..len {
            dp[i][i] = true;
        }

        for i in (0..len - 1).rev() {
            for j in i + 1..len {
                if s[i] == s[j] {
                    if j - i == 1 || dp[i + 1][j - 1] {
                        dp[i][j] = true;
                        if j - i + 1 > max_len {
                            start = i;
                            max_len = j - i + 1;
                        }
                    }
                }
            }
        }

        std::str::from_utf8(&s[start..start + max_len]).unwrap().to_string()
    }
}

// impl Solution {
//     pub fn is_palidormic_substring(s: &str) -> bool {
//         let s = s.as_bytes();
//         let mut left = 0;
//         let mut right = s.len() - 1;

//         while left < right {
//             if s[left] != s[right] {
//                 return false;
//             }
//             left += 1;
//             right -= 1;
//         }
//         true
//     }

//     pub fn longest_palindrome(s: String) -> String {
//         let s = s.as_bytes();
//         let mut p = vec![];

//         for i in 0..s.len() {
//             for j in i..s.len() {
//                 if Solution::is_palidormic_substring(std::str::from_utf8(&s[i..=j]).unwrap()) {
//                     if &s[i..=j].len() > &p.len() {
//                         p = s[i..=j].to_vec();

//                     }
//                 }
//             }
//         }
//         p.to_vec().iter().map(|&c| c as char).collect()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "babad".to_string();
        let result = "bab".to_string();
        assert_eq!(Solution::longest_palindrome(s), result);
    }

    #[test]
    fn test_2() {
        let s = "cbbd".to_string();
        let result = "bb".to_string();
        assert_eq!(Solution::longest_palindrome(s), result);
    }

    #[test]
    fn test_3() {
        let s = "a".to_string();
        let result = "a".to_string();
        assert_eq!(Solution::longest_palindrome(s), result);
    }

    #[test]
    fn test_4() {
        let s = "ac".to_string();
        let result = "a".to_string();
        assert_eq!(Solution::longest_palindrome(s), result);
    }
}
