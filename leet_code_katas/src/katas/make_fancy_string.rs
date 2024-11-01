struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut max = 1; 
        let by = s.as_bytes();
        let mut result = String::new();

        result.push(by[0] as char);
        for i in 1..by.len() {
            if by[i] == by[i - 1] {
                max += 1;
            } else {
                max = 1;
            }

            if max < 3 {
                result.push(by[i] as char);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_fancy_string_1() {
        assert_eq!(Solution::make_fancy_string("leeetcode".to_string()), "leetcode".to_string());
    }

    #[test]
    fn make_fancy_string_2() {
        assert_eq!(Solution::make_fancy_string("aaabaaaa".to_string()), "aabaa".to_string());
    }

    #[test]
    fn make_fancy_string_3() {
        assert_eq!(Solution::make_fancy_string("aab".to_string()), "aab".to_string());
    }
}
