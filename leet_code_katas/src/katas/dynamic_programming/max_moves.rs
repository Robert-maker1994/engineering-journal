use super::DynamicProgramming;

impl DynamicProgramming {
    /// This module provides a solution to find the maximum number of moves in a grid.
    /// 
    /// # Algorithm
    /// 
    /// 1. **Initialize Variables:**
    ///    - `let m = grid.len();` and `let n = grid[0].len();` get the dimensions of the grid.
    ///    - `let mut memo = vec![vec![-1; n]; m];` initializes the memoization table with `-1` to indicate unvisited cells.
    ///    - `let mut max_moves = 0;` initializes the variable to store the maximum number of moves.
    /// 
    /// 2. **Define DFS Function:**
    ///    - The `dfs` function takes the grid, memoization table, and current cell coordinates as input.
    ///    - It checks if the current cell has already been visited using the memoization table.
    ///    - It explores all possible moves (up-right, right, down-right) and recursively calls `dfs` for valid moves.
    ///    - It updates the memoization table with the maximum number of moves from the current cell.
    /// 
    /// 3. **Iterate Over First Column:**
    ///    - The outer loop iterates over all cells in the first column and calls the `dfs` function to find the maximum number of moves starting from each cell.
    /// 
    /// 4. **Return the Maximum Moves:**
    ///    - The function returns the maximum number of moves found.
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut memo = vec![vec![-1; n]; m];
        let mut max_moves = 0;

        fn dfs(grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
            if memo[row][col] != -1 {
                return memo[row][col];
            }

            let directions = [(-1, 1), (0, 1), (1, 1)];
            let mut max_move = 0;

            for &(dr, dc) in &directions {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;

                if new_row >= 0 && new_row < grid.len() as isize && new_col < grid[0].len() as isize {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if grid[new_row][new_col] > grid[row][col] {
                        max_move = max_move.max(1 + dfs(grid, memo, new_row, new_col));
                    }
                }
            }

            memo[row][col] = max_move;
            max_move
        }

        for row in 0..m {
            max_moves = max_moves.max(dfs(&grid, &mut memo, row, 0));
        }

        max_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![5, 4, 9, 3],
            vec![10, 9, 13, 15],
        ];
        assert_eq!(DynamicProgramming::max_moves(grid), 3);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
        assert_eq!(DynamicProgramming::max_moves(grid), 0);
    }
}
