use super::solution::Solution;

impl Solution { 
    /// This function finds the median of two sorted arrays.
    /// 
    /// # Arguments
    /// 
    /// * `nums1` - A vector of integers representing the first sorted array.
    /// * `nums2` - A vector of integers representing the second sorted array.
    /// 
    /// # Returns
    /// 
    /// * `f64` - The median of the two sorted arrays.
    /// 
    /// # Explanation
    /// 
    /// The function first merges the two input arrays into a single array `nums`.
    /// It then sorts the merged array. The length of the merged array is calculated
    /// to determine if it is even or odd.
    /// 
    /// If the length of the merged array is even, the median is calculated as the
    /// average of the two middle elements. If the length is odd, the median is the
    /// middle element of the sorted array.
    /// 
    /// # Algorithm
    /// 
    /// 1. Merge `nums1` and `nums2` into a single array `nums`.
    /// 2. Sort the merged array `nums`.
    /// 3. Calculate the length of the merged array.
    /// 4. If the length is even, return the average of the two middle elements as the median.
    /// 5. If the length is odd, return the middle element as the median.
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1;
        nums.extend(nums2);
        nums.sort();
        let len = nums.len();
        if len % 2 == 0 {
            let s = nums[len / 2 - 1] + nums[len / 2];
            return s as f64 / 2.0;
        } else {
            nums[len / 2] as f64
        }

    }
}
