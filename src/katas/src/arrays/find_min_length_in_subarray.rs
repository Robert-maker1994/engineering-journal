use super::Arrays;

// Todo descriptions
impl Arrays {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
            let n = arr.len();
            let mut right = n - 1;
    
            // Find the point where the array stops being non-decreasing from the end
            while right > 0 && arr[right] >= arr[right - 1] {
                right -= 1;
            }
    
            // If the entire array is non-decreasing, no need to remove any subarray
            if right == 0 {
                return 0;
            }
    
            let mut ans = right as i32;
            let mut left = 0;
    
            // Try to extend the valid subarray from the left side
            while left < right && (left == 0 || arr[left - 1] <= arr[left]) {
                // Find the next valid position on the right where arr[left] <= arr[right]
                while right < n && arr[left] > arr[right] {
                    right += 1;
                }
                // Calculate the length of the removed subarray
                ans = ans.min((right - left - 1) as i32);
                left += 1;
            }
    
            ans
        
}
}