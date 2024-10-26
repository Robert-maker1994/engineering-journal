// Problem: Minimum Add to Make Parentheses Valid
// Link: https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/

pub  

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut count = 0;

        for c in s.chars() {
            if c == '(' {
                stack.push(c);
            } else if c == ')' {
                if stack.is_empty() {
                    count += 1;
                } else {
                    stack.pop();
                }
            }
        }

        count + stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "())".to_string();
        let res = 1;
        assert_eq!(Solution::min_add_to_make_valid(s), res);
    }

    #[test]
    fn test_2() {
        let s = "(((".to_string();
        let res = 3;
        assert_eq!(Solution::min_add_to_make_valid(s), res);
    }

    #[test]
    fn test_3() {
        let s = "()".to_string();
        let res = 0;
        assert_eq!(Solution::min_add_to_make_valid(s), res);
    }
}