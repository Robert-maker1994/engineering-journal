
use super::BitManipulation;

impl BitManipulation {
 /// Largest Combination With Bitwise AND Greater Than Zero   
///  The bitwise AND of an array nums is the bitwise AND of all integers in nums.
/// For example, for nums = [1, 5, 3], the bitwise AND is equal to 1 & 5 & 3 = 1.
/// Also, for nums = [7], the bitwise AND is 7.
/// You are given an array of positive integers candidates. Evaluate the bitwise AND of every combination of numbers of candidates. Each number in candidates may only be used once in each combination.
/// Return the size of the largest combination of candidates with a bitwise AND greater than 0.
/// 
/// ### Solution: We are using the outOuter Loop: The outer loop iterates over each bit position (from 0 to 30). We are using a 31-bit range because the problem involves positive integers and for typical 32-bit integers, 30 bits are enough to represent them.
/// - Inner Loop: For each number in the candidates array, we check if the current bit (bit) is set using the bitwise AND operation: num & (1 << bit). If it is set, we increment the count.
/// - Updating Maximum: After counting how many numbers have the current bit set, we update the max_combination_size with the largest count found.
/// - Return the Result: The largest count is the answer, as it represents the largest subset of numbers that share at least one common bit set to 1.
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut max_combination_size = 0;


    for bit in 0..31 {
        let mut count = 0;
        
        for &num in &candidates {
            if num & (1 << bit) != 0 {
                count += 1;
            }
        }
        max_combination_size = max_combination_size.max(count);
    }
    
    max_combination_size
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_combination_1() {
        assert_eq!(BitManipulation::largest_combination(vec![16,17,71,62,12,24,14]),4)
    }
}