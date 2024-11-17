use super::Arrays;
/// TODO documation and organising 
impl Arrays {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = *quantities.iter().max().unwrap();
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::can_pack(n, &quantities, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
        }
        fn can_pack(n: i32, quantities: &Vec<i32>, max_operations: i32) -> bool {
            let mut operations = 0;
            for &q in quantities.iter() {
                operations += (q + max_operations - 1) / max_operations;
            }
            operations <= n
        }

}