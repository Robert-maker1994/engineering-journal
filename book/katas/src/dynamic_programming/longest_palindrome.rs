use super::DynamicProgramming;

impl DynamicProgramming {    
    /// Finds the longest palindromic substring in the given string `s`.
    ///
    /// # Arguments
    ///
    /// * `s` - A `String` for which the longest palindromic substring is to be found.
    ///
    /// # Returns
    ///
    /// A `String` representing the longest palindromic substring in `s`.
    /// 
    /// # Explanation
    ///
    /// The function uses a dynamic programming approach to find the longest palindromic substring.
    /// It iterates through each character in the string and considers it as the center of a potential palindrome.
    /// For each center, it expands outwards to check for palindromes of both odd and even lengths.
    /// The helper function `expand_around_center` is used to determine the length of the palindrome centered at a given index.
    /// The function keeps track of the start and end indices of the longest palindrome found during the iteration.
    /// Finally, it extracts and returns the longest palindromic substring using these indices.
pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return "".to_string();
    }

    let s = s.as_bytes();
    let len = s.len();
    let (mut start, mut end) = (0, 0);

    for i in 0..len {
        let (len1, len2) = (Self::expand_around_center(s, i, i), Self::expand_around_center(s, i, i + 1));
        let max_len = len1.max(len2);
        if max_len > end - start {
            start = i.saturating_sub((max_len - 1) / 2);
            end = i + max_len / 2;
        }
    }

    std::str::from_utf8(&s[start..=end]).unwrap().to_string()
}

fn expand_around_center(s: &[u8], left: usize, right: usize) -> usize {
    let (mut l, mut r) = (left as isize, right as isize);
    while l >= 0 && r < s.len() as isize && s[l as usize] == s[r as usize] {
        l -= 1;
        r += 1;
    }
    (r - l - 1) as usize
}
}