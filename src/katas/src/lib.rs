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


pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
           
    let n = code.len();
    if k == 0 {
        return vec![0; n];
    }

    (0..n).map(|i| {
        if k > 0 {
            (1..=k).map(|j| code[(i + j as usize) % n]).sum()
        } else {
            (1..=(-k)).map(|j| code[(i + n - j as usize) % n]).sum()
        }
    }).collect()

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_positive_k() {
        let code = vec![5, 7, 1, 4];

        assert_eq!(code, vec![12,10,16,13]);
    }
}