use std::collections::HashMap;

 

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut h = vec![];
        let mut max_len = 0;
        let mut start = 0;

        for (i, c) in s.chars().enumerate() {
            if h.contains(&c) {
                start = start.max(i + 1);
            }
            h.push(c);
            max_len = max_len.max(i - start + 1);
        }

        max_len.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
