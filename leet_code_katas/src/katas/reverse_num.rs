 /// Reverses the digits of a given 32-bit signed integer.
///
/// # Arguments
///
/// * `x` - A 32-bit signed integer whose digits are to be reversed.
///
/// # Returns
///
/// * A 32-bit signed integer with its digits reversed. If the reversed integer overflows, it returns 0.
///
/// # Example
///
/// ```
/// let result = Solution::reverse(123);
/// assert_eq!(result, 321);
///
/// let result = Solution::reverse(-123);
/// assert_eq!(result, -321);
///
/// let result = Solution::reverse(120);
/// assert_eq!(result, 21);
/// ```
///
/// # Explanation
///
/// The function first checks if the input integer `x` is negative. If it is, it sets a flag `is_negative` to true.
/// It then converts the absolute value of `x` to a string, reverses the characters, and parses it back to an integer.
/// If the original number was negative, it negates the reversed integer.
/// The function is executed in a separate thread to handle potential panics gracefully, and the result is returned.
///
/// Note: The use of threading here is unconventional and not necessary for this problem. It might be better to handle
/// potential errors directly within the function without spawning a new thread.
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        std::thread::Builder::new()
        .spawn(move ||{
        // Handle negative numbers
        let is_negative = x < 0;
        let mut x = x.abs().to_string().chars().rev().collect::<String>().parse::<i32>().unwrap_or(0);
        
        if is_negative {
            x = -x;
        }
        
        x
        }).unwrap().join().unwrap()
    }
}
