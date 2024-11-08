use super::Arrays;
impl Arrays {
    /// # Determines if the given array is a valid mountain array.
    ///
    /// A mountain array is defined as an array that:
    /// - Has at least 3 elements.
    /// - There exists some index `i` (0 < i < arr.length - 1) such that:
    ///   - `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
    ///   - `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`
    ///
    /// # Arguments
    ///
    /// * `arr` - A vector of integers representing the array to be checked.
    ///
    /// # Returns
    ///
    /// * `true` if the array is a valid mountain array, `false` otherwise.
    ///
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }

        let mut i = 0;
        // When the condition is met, the loop will stop. For this instance the condition is: arr[i] < arr[i + 1]
        while i + 1 < n && arr[i] < arr[i + 1] {
            i += 1;
        }

        if i == 0 || i == n - 1 {
            return false;
        }

        // this will start in the decreasing part of the array
        while i + 1 < n && arr[i] > arr[i + 1] {
            i += 1;
        }

        i == n - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests for the `valid_mountain_array` function.
    #[test]
    fn test_valid_mountain_array() {
        assert_eq!(
            Arrays::valid_mountain_array(vec![0, 1, 2, 3, 4, 5, 4, 6, 2, 1]),
            true
        );
    }
}
