struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1;
        nums.extend(nums2);
        nums.sort();
        let len = nums.len();
        if len % 2 == 0 {
            let s = nums[len / 2 - 1] + nums[len / 2];
            return s as f / 2.0;
        } else {
            nums[len / 2] as f64
        }

    }
}
