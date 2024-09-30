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



// let mut nums1 = vec![1, 1, 1, 2, 2, 3];
// let expected_nums1 = vec![1, 1, 2, 2, 3];
// let k1 = remove_duplicates(&mut nums1);

// assert_eq!(k1, expected_nums1.len() as i32);
// for i in 0..k1 as usize {
//     assert_eq!(nums1[i], expected_nums1[i]);
// }

// println!("{:?}", &nums1[..k1 as usize]); 
