use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    let mut heap = BinaryHeap::new();

    for &num in &nums {
        heap.push(num);
    }

    println!("{:?}", heap);

    let mut score: i64 = 0;
    for _ in 0..k {

        if let Some(max_element) = heap.pop() {
            score += max_element as i64;
      
            let new_value = (max_element as f64 / 3.0).ceil() as i32;
      
            heap.push(new_value);
        }
    }

    score
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_kelements() {
        assert_eq!(Solution::max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
        assert_eq!(Solution::max_kelements(vec![1,10,3,3,3], 3), 17);
        assert_eq!(Solution::max_kelements(vec![9, 8, 7, 6, 5], 3), 24);    
    }

}