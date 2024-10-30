struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut count = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    dp[i][j] = 1;
                    if i > 0 && j > 0 {
                        dp[i][j] += dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1]));
                    }
                    count += dp[i][j];
                }
            }
        }

        count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_squares() {
        assert_eq!(Solution::count_squares(vec![vec![0,1,1,1], vec![1,1,1,1], vec![0,1,1,1]]), 15);
        assert_eq!(Solution::count_squares(vec![vec![1,0,1], vec![1,1,0], vec![1,1,0]]), 7);
    }
}