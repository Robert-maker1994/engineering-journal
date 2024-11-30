struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut left = 0;
        let mut right = 0;

        if edges.len() == 2 {
            return -1;
        }

        edges.iter().fold( -1, |mut f, v| {
            if right == v[0] {
                f = right;
            }

            if left == v[1] {
                f = left;
            }
            left = v[0];
            right = v[1];
            println!("right {}, left {}, f: {}", right, left, f);
            f
        })
    }
}

fn main() {
  

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_center() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![4, 2]];
        assert_eq!(Solution::find_center(edges), 2);
    }
}