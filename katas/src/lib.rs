use std::{collections::HashMap};

pub mod arrays;
pub mod backtracking;
pub mod bit_manipulation;
pub mod data_structures;
pub mod depth_first_search;
pub mod dynamic_programming;
pub mod hash_table;
pub mod numbers;
pub mod queue;
pub mod strings;


fn maximum_subarray_sum(vec: Vec<i32>, ) {
    let mut ans = 0;
    let mut current_sum = 0;
    let mut begin = 0;
    let mut end = 0;
    let mut num_to_index: HashMap<i32, i32> = HashMap::new();

    while end < vec.len() {
        current_sum = vec[begin as usize];
        let last_occurrence = num_to_index.get(&current_sum);
        //  if current window already has number or if window is too big, adjust window
       
    }
    ans

}