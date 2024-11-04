use std::vec;

pub struct Solution;

/// This function finds the longest palindromic substring in a given string `s`.
///
/// # Arguments
///
/// * `s` - A `String` input for which the longest palindromic substring is to be found.
///
/// # Returns
///
/// * A `String` representing the longest palindromic substring in the input string.
///
/// # Explanation
///
/// The function uses dynamic programming to solve the problem. It maintains a 2D vector `dp`
/// where `dp[i][j]` is `true` if the substring `s[i..=j]` is a palindrome. The function iterates
/// over all possible substrings and updates the `dp` table accordingly. It also keeps track of
/// the starting index and the maximum length of the longest palindromic substring found so far.
///
/// The algorithm works as follows:
/// 1. Initialize a 2D vector `dp` with `false` values.
/// 2. Set `dp[i][i]` to `true` for all `i` because a single character is always a palindrome.
/// 3. Iterate over the string in reverse order to ensure that when checking substring `s[i..=j]`,
///    the values for `dp[i+1][j-1]` are already computed.
/// 4. For each pair of indices `(i, j)`, if `s[i] == s[j]` and the substring `s[i+1..=j-1]` is a
///    palindrome (or `j - i == 1` which means the substring length is 2), then `s[i..=j]` is a
///    palindrome. Update the `dp` table and check if this palindrome is the longest found so far.
/// 5. Finally, extract the longest palindromic substring using the starting index and maximum length.
///
/// The function returns the longest palindromic substring as a `String`.
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
