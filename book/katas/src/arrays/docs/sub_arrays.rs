pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let len = nums.len();
    let mut results: Vec<Vec<i32>> = vec![];

    for start in 0..=len {
        let mut isConsecutiveAndSorted  = true;
        let mut sub_array: Vec<i32> = vec![];
        let i = start + k as usize;
        if i < len {
            for sub in start..i {
                println!("sub {}", nums[sub]);
                sub_array.push(nums[sub]);
            }
            // Access  the surray to do stuff 
            
            println!("sub array {:?}", sub_array);
            
            results.push(sub_array);
        }
    }
    results[0].clone()
}