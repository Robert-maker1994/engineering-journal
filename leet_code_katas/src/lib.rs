struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let doubled_s = s.clone() + &s;
        doubled_s.contains(&goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotate_string() {
        assert_eq!(Solution::rotate_string("abcde".to_string(), "cdeab".to_string()), true)
    }
    #[test]
    fn rotate_string_1() {
        assert_eq!(Solution::rotate_string("abcde".to_string(), "abced".to_string()), false)
    }
}