use super::Strings;

impl Strings {
    /// This function calculates the minimum number of parentheses that need to be added 
    /// to make a given string of parentheses valid. A string of parentheses is considered 
    /// valid if every opening parenthesis '(' has a corresponding closing parenthesis ')'.
    ///
    /// # Arguments
    ///
    /// * `s` - A string containing only the characters '(' and ')'.
    ///
    /// # Returns
    ///
    /// * `i32` - The minimum number of parentheses that need to be added to make the string valid.
    ///
    /// # Explanation
    ///
    /// The function uses a stack to keep track of unmatched opening parentheses. It iterates 
    /// through each character in the string:
    ///
    /// - If the character is '(', it is pushed onto the stack.
    /// - If the character is ')', the function checks if the stack is empty:
    ///   - If the stack is empty, it means there is no matching opening parenthesis for this 
    ///     closing parenthesis, so the `count` is incremented.
    ///   - If the stack is not empty, it means there is a matching opening parenthesis, so 
    ///     the top of the stack is popped.
    ///
    /// After iterating through the string, any remaining opening parentheses in the stack 
    /// are unmatched, so their count is added to the `count` variable.
    ///
    /// The final result is the sum of the unmatched closing parentheses (`count`) and the 
    /// unmatched opening parentheses (`stack.len()`).
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
        assert_eq!(Strings::min_add_to_make_valid(s), res);
    }

    #[test]
    fn test_2() {
        let s = "(((".to_string();
        let res = 3;
        assert_eq!(Strings::min_add_to_make_valid(s), res);
    }

    #[test]
    fn test_3() {
        let s = "()".to_string();
        let res = 0;
        assert_eq!(Strings::min_add_to_make_valid(s), res);
    }
}