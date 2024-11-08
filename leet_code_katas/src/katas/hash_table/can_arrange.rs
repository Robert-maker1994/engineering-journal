use super::HashTable;

impl HashTable {
/// Determines if the array can be rearranged such that the sum of every pair of elements is divisible by `k`.
///
/// # Arguments
///
/// * `arr` - A vector of integers.
/// * `k` - An integer by which the sum of pairs should be divisible.
///
/// # Returns
///
/// * `true` if the array can be rearranged to satisfy the condition, otherwise `false`.
///
/// # Explanation
///
/// The function first calculates the frequency of each remainder when the elements of the array are divided by `k`.
/// It then checks if the frequency of elements with a remainder of 0 is even, as these elements need to pair among themselves.
/// For other remainders, it checks if the frequency of a remainder `i` matches the frequency of its complement `k - i`.
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut mod_freq = vec![0; k as usize];

        // Count the remainders
        for &num in &arr {
            let remainder = ((num % k) + k) % k;
            mod_freq[remainder as usize] += 1;
        }

        // Check if the count of remainder 0 is even
        if mod_freq[0] % 2 != 0 {
            return false;
        }

        // Check if the counts of other remainders match their complements
        for i in 1..(k as usize) {
            if mod_freq[i] != mod_freq[k as usize - i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_arrange() {
        assert_eq!(HashTable::can_arrange(vec![1, 2, 3, 4, 5, 10, -10, -5, -4, -3, -2, -1], 5), true);
        assert_eq!(HashTable::can_arrange(vec![1, 2, 3, 4, 5, 6], 7), true);
        assert_eq!(HashTable::can_arrange(vec![1, 2, 3, 4, 5, 6], 10), false);
        assert_eq!(HashTable::can_arrange(vec![1, 2, 3, 4, 5, -6], 5), false);
        assert_eq!(HashTable::can_arrange(vec![-1, 1, -2, 2, -3, 3, -4, 4], 3), true);
    }
}
