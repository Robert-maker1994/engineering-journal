//! Highest Scoring Word

///Given a string of words, you need to find the highest scoring word.
/// Each letter of a word scores points according to its position in the alphabet: a = 1, b = 2, c = 3 etc.
/// For example, the score of abad is 8 (1 + 2 + 1 + 4).
/// You need to return the highest scoring word as a string.
/// If two words score the same, return the word that appears earliest in the original string.
/// All letters will be lowercase and all inputs will be valid.

// My Example
pub fn highest_scoring_word(s: &str) -> &str {
    let mut highest_score = 0;
    let mut highest_word = "";
    for word in s.split_whitespace() {
        let score = word.chars().map(|c| c as u32 - 96).sum::<u32>();
        if score > highest_score {
            highest_score = score;
            highest_word = word;
        }
    }
    highest_word
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_scoring_word() {
        assert_eq!(highest_scoring_word("aaa b"), "aaa");

        assert_eq!(highest_scoring_word("man i need a taxi up to ubud"), "taxi");
        assert_eq!(
            highest_scoring_word("what time are we climbing up the volcano"),
            "volcano"
        );
        assert_eq!(highest_scoring_word("take me to semynak"), "semynak");
        assert_eq!(
            highest_scoring_word("massage yes massage yes massage"),
            "massage"
        );
        assert_eq!(
            highest_scoring_word("take two bintang and a dance please"),
            "bintang"
        );
        assert_eq!(highest_scoring_word("aa b"), "aa");
        assert_eq!(highest_scoring_word("b aa"), "b");
        assert_eq!(highest_scoring_word("bb d"), "bb");
        assert_eq!(highest_scoring_word("d bb"), "d");
    }
}
