///! - **String**: Algorithms and problems related to String manipulation.
pub struct Strings;

/// Compresses a string by replacing consecutive repeating characters with the character followed by the count.
pub mod compressed_string;

/// Checks if a given sentence is circular.
/// A sentence is considered circular if the last character of each word matches the first character of the next word,
/// and the last character of the last word matches the first character of the first word.
pub mod is_circular_sentence;

/// Removes consecutive duplicate characters from a string.
pub mod make_fancy_string;

/// Calculates the minimum number of parentheses that need to be added to make a given string of parentheses valid.
pub mod min_add_to_make_valid;

/// Finds the minimum number of changes required to make a string valid.
pub mod min_changes;