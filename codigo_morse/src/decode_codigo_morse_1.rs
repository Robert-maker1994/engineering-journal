use std::collections::HashMap;

pub fn decode_morse(encoded: &str) -> String {
    let morse_code_dict = morse_code();
    let mut decoded = String::new();
    for word in encoded.split("   ") {
        for symbol in word.split_whitespace() {
            decoded.push(*morse_code_dict.get(symbol).unwrap());
        }
        decoded.push(' ');
    }
    decoded.pop(); // Remove the trailing space
    decoded
}
  

pub fn morse_code() -> HashMap<String, char> {
    let mut morse_code_dict = HashMap::new();
    morse_code_dict.insert(".-".to_string(), 'A');
    morse_code_dict.insert("-...".to_string(), 'B');
    morse_code_dict.insert("-.-.".to_string(), 'C');
    morse_code_dict.insert("-..".to_string(), 'D');
    morse_code_dict.insert(".".to_string(), 'E');
    morse_code_dict.insert("..-.".to_string(), 'F');
    morse_code_dict.insert("--.".to_string(), 'G');
    morse_code_dict.insert("....".to_string(), 'H');
    morse_code_dict.insert("..".to_string(), 'I');
    morse_code_dict.insert(".---".to_string(), 'J');
    morse_code_dict.insert("-.-".to_string(), 'K');
    morse_code_dict.insert(".-..".to_string(), 'L');
    morse_code_dict.insert("--".to_string(), 'M');
    morse_code_dict.insert("-.".to_string(), 'N');
    morse_code_dict.insert("---".to_string(), 'O');
    morse_code_dict.insert(".--.".to_string(), 'P');
    morse_code_dict.insert("--.-".to_string(), 'Q');
    morse_code_dict.insert(".-.".to_string(), 'R');
    morse_code_dict.insert("...".to_string(), 'S');
    morse_code_dict.insert("-".to_string(), 'T');
    morse_code_dict.insert("..-".to_string(), 'U');
    morse_code_dict.insert("...-".to_string(), 'V');
    morse_code_dict.insert(".--".to_string(), 'W');
    morse_code_dict.insert("-..-".to_string(), 'X');
    morse_code_dict.insert("-.--".to_string(), 'Y');
    morse_code_dict.insert("--..".to_string(), 'Z');
    morse_code_dict.insert("-----".to_string(), '0');
    morse_code_dict.insert(".----".to_string(), '1');
    morse_code_dict.insert("..---".to_string(), '2');
    morse_code_dict.insert("...--".to_string(), '3');
    morse_code_dict.insert("....-".to_string(), '4');
    morse_code_dict.insert(".....".to_string(), '5');
    morse_code_dict.insert("-....".to_string(), '6');
    morse_code_dict.insert("--...".to_string(), '7');
    morse_code_dict.insert("---..".to_string(), '8');
    morse_code_dict.insert("----.".to_string(), '9');
    morse_code_dict
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }
}
