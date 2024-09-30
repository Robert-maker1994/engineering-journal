fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut write_index = 1;
    let mut count = 1;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            count += 1;
        } else {
            count = 1;
        }

        if count <= 2 {
            nums[write_index] = nums[i];
            write_index += 1;
        }
    };

    write_index as i32
}
