use super::Arrays;

impl Arrays {

    /// Solution: 
    /// Check if the array is already sorted
    /// Group numbers by the count of 1's in their binary representation
    /// Sort each group individually
    /// Merge the sorted groups back into the nums array
    /// Check if the array is sorted
        pub fn can_sort_array(nums: Vec<i32>) -> bool {
        if nums.windows(2).all(|w| w[0] <= w[1]) {
            return true;
        }

        let mut groups: std::collections::HashMap<u32, Vec<i32>> = std::collections::HashMap::new();

        for &num in &nums {
            let count_ones = num.count_ones();
            groups.entry(count_ones).or_insert(Vec::new()).push(num);
        }

        for group in groups.values_mut() {
            group.sort();
        }

        let mut sorted_nums: Vec<i32> = Vec::with_capacity(nums.len());
        for count_ones in 0..=32 {
            if let Some(group) = groups.get(&count_ones) {
                sorted_nums.extend(group);
            }
        }

        sorted_nums.windows(2).all(|w| w[0] <= w[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_sort_array_1() {
        assert_eq!(Arrays::can_sort_array(vec![8, 4, 2, 30, 15]), true)
    }
    #[test]
    fn test_can_sort_array_2() {
        assert_eq!(Arrays::can_sort_array(vec![1, 2, 3, 4, 5]), true)
    }
    
    #[test]
    fn test_can_sort_array_3() {
        assert_eq!(Arrays::can_sort_array(vec![3, 16, 8, 4, 2]), false)
    }

    #[test]
    fn test_can_sort_array_4() {
        assert_eq!(Arrays::can_sort_array(vec![20, 16]), false)
    }
    #[test]
    fn test_can_sort_array_5() {
        assert_eq!(Arrays::can_sort_array(vec![136,256,10]), false)
    }

}
