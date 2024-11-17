
/// Given an integer array nums and an integer k, return the length of the shortest non-empty subarray of nums with a sum of at least k. If there is no such subarray, return -1.

/// A subarray is a contiguous part of an array.
/// Solution: 
/// - Calculate the prefix sums of the input array `nums`.
/// - Initialize a deque to store indices and a variable to track the length of the shortest subarray.
/// - Iterate through the prefix sums:
///     - Remove indices from the front of the deque while the difference between the current prefix sum and the prefix sum at the front of the deque is at least `k`.
///     - Update the length of the shortest subarray if a valid subarray is found.
///     - Remove indices from the back of the deque while the prefix sum at the back is greater than or equal to the current prefix sum.
///     - Add the current index to the back of the deque.
/// - Return the length of the shortest subarray if found, otherwise return -1.
use super::Queues;
use std::collections::VecDeque;
use std::iter::once;

impl Queues {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let sum_iter = nums.iter().scan(0, |a, &n| {*a += n as i64; Some(*a)});
        let pfx_sums = once(0).chain(sum_iter).collect::<Vec<_>>();

        let mut indices = VecDeque::new();
        let mut sub_len = usize::MAX;

        let k = k as i64;
        
        for (i, &sum) in pfx_sums.iter().enumerate() {

            while indices.front().map_or(false, |&j| sum - pfx_sums[j] >= k) {
                sub_len = sub_len.min(i - indices.pop_front().unwrap());
            }

            while indices.back().map_or(false, |&j| pfx_sums[j] >= sum) {
                indices.pop_back();
            }

            indices.push_back(i);
        }

        if sub_len == usize::MAX { 
            -1 
        } else { 
            sub_len as i32
        }
        }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_shortest_subarray_single_element() {
        let nums = vec![1];
        let k = 1;
        assert_eq!(Queues::shortest_subarray(nums, k), 1);
    }

}

