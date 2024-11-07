use crate::katas::solution::Solution;

impl Solution {
    /// You are given a 0-indexed binary string s having an even length.
    /// A string is beautiful if it's possible to partition it into one or more substrings such that:
    /// Each substring has an even length.
    /// Each substring contains only 1's or only 0's.
    /// You can change any character in s to 0 or 1.
    /// Return the minimum number of changes required to make the string s beautiful.

    /// 0-Index is an array or an index that starts

    /// Solution: I need to chunk the string to into 1´s or 0´s with an even amount of letters per sub string. For this I'm going to have to have a few variable 
    pub fn min_changes(s: String) -> i32 {
        let len = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut odd = Vec::new();
        let mut i = 1;
        let mut start = 0;
        let mut count = 0;

        while i < len {
            while i < len && chars[i] == chars[i - 1] {
                i += 1;
            }

            if (i - start) % 2 != 0 {
                odd.push(count);
            }
            
            count += 1;
            start = i;
            i += 1;
        }

        if (i - start) % 2 != 0 {
            odd.push(count);
        }

        let mut min_changes = 0;

        for pos in (1..odd.len()).step_by(2) {
            min_changes += odd[pos] - odd[pos - 1];
        }

        min_changes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_changes_1() {
        //
        assert_eq!(Solution::min_changes("1001".to_string()), 2);
    }

    #[test]
    fn min_changes_2() {
        assert_eq!(Solution::min_changes("0000".to_string()), 0);
    }

    #[test]
    fn min_changes_3() {
        assert_eq!(Solution::min_changes("10".to_string()), 1);
    }
    #[test]
    fn min_changes_4() {
        assert_eq!(Solution::min_changes("11000111".to_string()), 1);
    }

}
