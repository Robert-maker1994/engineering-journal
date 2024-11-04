use super::solution::Solution;
impl Solution {

/// Checks if a given sentence is circular.
/// 
/// A sentence is considered circular if the last character of each word matches the first character of the next word,
/// and the last character of the last word matches the first character of the first word.
/// 
/// # Arguments
/// 
/// * `sentence` - A `String` containing the sentence to be checked.
/// 
/// # Returns
/// 
/// * `bool` - Returns `true` if the sentence is circular, otherwise returns `false`.
/// # Explanation
/// 
/// The function splits the sentence into words and checks if the last character of each word matches the first character of the next word.
/// It also ensures that the last character of the last word matches the first character of the first word to complete the circular check.
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        let n = words.len();

        if n == 0 {
            return false;
        }

        for i in 0..n {
            let current_word = words[i];
            let next_word = words[(i + 1) % n];

            let current_last_char = current_word.chars().last().unwrap();
            let next_first_char = next_word.chars().next().unwrap();

            if current_last_char != next_first_char {
                return false;
            }
        }

        true
    }
}
