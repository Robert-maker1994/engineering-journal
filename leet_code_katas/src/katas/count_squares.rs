use super::solution::Solution;
impl Solution {
/// This function counts the number of square submatrices with all ones in a given binary matrix.
/// 
/// # Arguments
/// 
/// * `matrix` - A 2D vector of integers representing the binary matrix.
/// 
/// # Returns
/// 
/// * An integer representing the total number of square submatrices with all ones.
/// 
/// # Explanation
/// 
/// The function uses dynamic programming to solve the problem. It initializes a 2D vector `dp`
/// of the same size as the input matrix to store the size of the largest square submatrix ending
/// at each cell. It also initializes a counter `count` to keep track of the total number of such
/// submatrices.
/// 
/// The function iterates over each cell in the matrix. If the cell contains a 1, it updates the
/// corresponding cell in the `dp` vector. If the cell is not on the first row or first column,
/// it adds the minimum value among the top, left, and top-left diagonal cells in the `dp` vector
/// to the current cell. This ensures that only square submatrices are counted.
/// 
/// Finally, the function adds the value of the current cell in the `dp` vector to the `count`
/// and returns the total count.
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