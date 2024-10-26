 
// We are going to folve this using backracking 
impl Solution {
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
        assert_eq!(Solution::max_unique_split("aa".to_string()), 1);
    }
}