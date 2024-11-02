
impl Solution {
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
