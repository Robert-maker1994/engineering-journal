use super::Backtracking;

impl Backtracking {
    /// This implementation provides a solution to the problem of finding the maximum number of unique substrings
    /// that a given string can be split into. The solution uses a backtracking approach to explore all possible
    /// ways to split the string and keeps track of the maximum number of unique substrings found.
    ///
    /// # Explanation
    ///
    /// The `Solution` struct contains two methods:
    ///
    /// 1. `backtrack`: This is a helper method that performs the backtracking. It takes the following parameters:
    ///    - `s`: A reference to the input string.
    ///    - `set`: A mutable reference to a `HashSet` that keeps track of the unique substrings found so far.
    ///    - `max`: A mutable reference to an integer that stores the maximum number of unique substrings found.
    ///    - `start`: The starting index for the current substring being considered.
    ///
    ///    The method works as follows:
    ///    - If `start` is equal to the length of the string, it means we have considered all possible substrings.
    ///      In this case, we update the `max` value if the size of the set is greater than the current `max`.
    ///    - For each possible substring starting from `start` to the end of the string, we check if the substring
    ///      is already in the set. If it is, we skip it.
    ///    - If the substring is not in the set, we add it to the set and make a recursive call to `backtrack` with
    ///      the next starting index.
    ///    - After the recursive call, we remove the substring from the set to backtrack and explore other possibilities.
    ///
    /// 2. `max_unique_split`: This is the main method that initializes the necessary data structures and calls the
    ///    `backtrack` method. It takes a string as input and returns the maximum number of unique substrings.
    ///
    /// # Backtracking
    ///
    /// Backtracking is a general algorithmic technique that incrementally builds candidates to the solutions and
    /// abandons a candidate (backtracks) as soon as it determines that the candidate cannot possibly be completed
    /// to a valid solution. In this implementation, backtracking is used to explore all possible ways to split the
    /// string into unique substrings. By recursively exploring each possibility and backtracking when necessary,
    /// we ensure that we find the maximum number of unique substrings.
    // We are going to this using backracking 
    fn backtrack(s: &str, set: &mut std::collections::HashSet<String>, max: &mut i32, start: usize) {

        if start == s.len() {
            println!("set{:?}, max: {}", set.len(), max);
            // We are going to compare the max value with the length of the set
            *max = std::cmp::max( set.len() as i32, *max);
            return;
        }
        
        for i in start..s.len() {
            // Grapes an inclusive range of the string
            let left = &s[start..=i];

            if set.contains(left) {
                continue;
            }
            // Insert the string into the set to be able to look up later.
            set.insert(left.to_string());

            // We make a recursive call to backtrack 
            Self::backtrack(s, set, max, i + 1);
            // After the recursive call we remove the last element from the set
            set.remove(left);

        }
    }

    pub fn max_unique_split(s: String) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut max = 0;
        
        Self::backtrack(&s, &mut set, &mut max, 0);

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_unique_split() {
        // assert_eq!(Solution::max_unique_split("ababccc".to_string()), 5);
        // assert_eq!(Solution::max_unique_split("aba".to_string()), 2);
        assert_eq!(Backtracking::max_unique_split("aa".to_string()), 1);
    }
}