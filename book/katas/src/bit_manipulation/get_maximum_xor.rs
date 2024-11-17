use super::BitManipulation;

impl BitManipulation {
    /// ### 1829. Maximum XOR for Each Query
    ///  You are given a sorted array nums of n non-negative integers and an integer maximumBit. You want to perform the following query n times:
    ///  Find a non-negative integer k < 2maximumBit such that nums[0] XOR nums[1] XOR ... XOR nums[nums.length-1] XOR k is maximized. k is the answer to the ith query.
    
    /// Remove the last element from the current array nums.
    /// Return an array answer, where answer[´i´] is the answer to the ith query.
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor_all = 0;
        for &num in &nums {
            // Makes the xor of all elements. 
            xor_all ^= num;
        }

        let max_k = (1 << maximum_bit) - 1;
        
        let mut result = Vec::new();
        let mut current_xor = xor_all;
        for i in (0..nums.len()).rev() {
            let k = current_xor ^ max_k;
            
            result.push(k);
            current_xor ^= nums[i];
        }

        result
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_maximum_xor() {
        assert_eq!(BitManipulation::get_maximum_xor(vec![0,1,1,3], 2), vec![0, 3, 2, 3]);
    }
    
    #[test]
    fn test_get_maximum_xor_1() {
        assert_eq!(BitManipulation::get_maximum_xor(vec![2, 3, 4, 7], 3), vec![5, 2, 6, 5]);
        assert_eq!(BitManipulation::get_maximum_xor(vec![0,1,2,2,5,7], 3), vec![4, 3, 6, 4, 6, 7]);

    }
}
