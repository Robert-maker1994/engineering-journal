use super::Strings;

impl Strings {
    /// Compresses a given string by counting consecutive characters and 
    /// appending the count followed by the character to the result string.
    /// 
    /// # Arguments
    /// 
    /// * `word` - A `String` that represents the input word to be compressed.
    /// 
    /// # Returns
    /// 
    /// A `String` that contains the compressed version of the input word.
    /// 
    /// # Example
    /// 
    /// ```
    /// let result = Solution::compressed_string("aaabbc".to_string());
    /// assert_eq!(result, "3a2b1c");
    /// ```
    /// 
    /// # Notes
    /// 
    /// - The function limits the count of consecutive characters to 9.
    /// - If a character appears more than 9 times consecutively, it will be split into multiple counts.
    pub fn compressed_string(word: String) -> String {
        let mut comp = String::new();
        let mut chars = word.chars().peekable();

        while let Some(&c) = chars.peek() {
            let mut count = 0;

            while count < 9 && chars.peek() == Some(&c) {
                chars.next();
                count += 1;
            }

            comp.push_str(&format!("{}{}", count, c));
        }
        comp
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compressed_string_1() {
        assert_eq!(Strings::compressed_string("aaabbbccc".to_string()), "3a3b3c".to_string());
    }

    #[test]
    fn test_compressed_string_2() {
        assert_eq!(Strings::compressed_string("aaaaaaaaaaa".to_string()), "9a2a".to_string());
    }

    #[test]
    fn test_compressed_string_3() {
        assert_eq!(Strings::compressed_string("aabbcc".to_string()), "2a2b2c".to_string());
    }
}
