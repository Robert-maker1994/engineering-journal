struct Solution;

impl Solution {
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
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 10, -10, -5, -4, -3, -2, -1], 5), true);
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7), true);
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10), false);
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, -6], 5), false);
        assert_eq!(Solution::can_arrange(vec![-1, 1, -2, 2, -3, 3, -4, 4], 3), true);
    }
}