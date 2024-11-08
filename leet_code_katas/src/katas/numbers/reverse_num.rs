use super::Numbers;

impl Numbers {
    /// Reverses the digits of an integer.
    ///
    /// # Arguments
    ///
    /// * `x` - An integer to be reversed.
    ///
    /// # Returns
    ///
    /// * `i32` - The reversed integer.
    ///
    /// # Explanation
    ///
    /// The function handles negative numbers by checking if the input is negative.
    /// It converts the absolute value of the integer to a string, reverses the string,
    /// and then parses it back to an integer. If the original number was negative,
    /// it negates the reversed integer.
    /// The function is executed in a separate thread to handle potential panics gracefully,
    /// and the result is returned.
    pub fn reverse(x: i32) -> i32 {
        std::thread::Builder::new()
            .spawn(move || {
                // Handle negative numbers
                let is_negative = x < 0;
                let mut x = x.abs().to_string().chars().rev().collect::<String>().parse::<i32>().unwrap_or(0);

                if is_negative {
                    x = -x;
                }

                x
            })
            .unwrap()
            .join()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_positive() {
        assert_eq!(Numbers::reverse(123), 321);
    }

    #[test]
    fn test_reverse_negative() {
        assert_eq!(Numbers::reverse(-123), -321);
    }

    #[test]
    fn test_reverse_zero() {
        assert_eq!(Numbers::reverse(0), 0);
    }

    #[test]
    fn test_reverse_overflow() {
        assert_eq!(Numbers::reverse(1534236469), 0); // This should handle overflow gracefully
    }
}