use std::collections::HashSet;

/// Leetcode 51: N-Queens
///
/// Intuition:
/// -----------
/// - The core idea is to place one queen per row, ensuring that no two queens
///   share the same column or diagonals.
/// - We use backtracking to try every column for each row, but we prune
///   invalid choices early by tracking:
///     1. `columns`: which columns are already occupied.
///     2. `diagonals1`: occupied main diagonals, identified by `row - col`.
///     3. `diagonals2`: occupied anti-diagonals, identified by `row + col`.
///
/// - Each valid board configuration is built as a vector of strings,
///   where 'Q' denotes a queen, and '.' denotes an empty square.
///
/// Algorithm:
/// -----------
/// - For each row:
///     - Try placing a queen in each column.
///     - If the position is safe:
///         - Mark the column and diagonals as occupied.
///         - Place the queen.
///         - Recurse to the next row.
///         - After recursion, backtrack: remove queen and unmark the positions.
///
/// Time Complexity:
/// ----------------
/// - O(N!) in the worst case, due to exploring permutations with pruning.
///
/// Space Complexity:
/// -----------------
/// - O(N^2) for storing results, O(N) for backtracking path and constraint sets.
pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        // Stores the final list of valid board configurations
        let mut results: Vec<Vec<String>> = Vec::new();

        // Mutable list tracking queen column positions per row
        let mut board: Vec<usize> = Vec::with_capacity(n as usize);

        // Sets to track used columns and diagonals
        let mut columns: HashSet<usize> = HashSet::new(); // Tracks used columns
        let mut diagonals1: HashSet<isize> = HashSet::new(); // Tracks (row - col)
        let mut diagonals2: HashSet<usize> = HashSet::new(); // Tracks (row + col)

        // Start recursive backtracking from row 0
        Self::backtrack(
            0,
            n as usize,
            &mut board,
            &mut columns,
            &mut diagonals1,
            &mut diagonals2,
            &mut results,
        );

        results
    }

    /// Recursively builds queen placements row by row
    fn backtrack(
        row: usize,                                        // Current row index
        n: usize,                                          // Board size (n x n)
        board: &mut Vec<usize>,                            // Board state: queen column per row
        columns: &mut std::collections::HashSet<usize>,    // Used columns
        diagonals1: &mut std::collections::HashSet<isize>, // Used (row - col)
        diagonals2: &mut std::collections::HashSet<usize>, // Used (row + col)
        results: &mut Vec<Vec<String>>,                    // Result collector
    ) {
        // Base case: all queens placed successfully
        if row == n {
            // Convert board of positions into Vec<String>
            results.push(Self::build_board(board, n));
            return;
        }

        // Try placing a queen in each column
        for col in 0..n {
            let d1: isize = row as isize - col as isize; // Main diagonal
            let d2: usize = row + col; // Anti-diagonal

            // Skip if any constraint is violated
            if columns.contains(&col) || diagonals1.contains(&d1) || diagonals2.contains(&d2) {
                continue;
            }

            // Choose: place the queen and mark constraints
            board.push(col);
            columns.insert(col);
            diagonals1.insert(d1);
            diagonals2.insert(d2);

            // Recurse to the next row
            Self::backtrack(row + 1, n, board, columns, diagonals1, diagonals2, results);

            // Undo: remove the queen and unmark constraints
            board.pop();
            columns.remove(&col);
            diagonals1.remove(&d1);
            diagonals2.remove(&d2);
        }
    }

    /// Converts the current queen positions into a board of 'Q' and '.'
    fn build_board(board: &Vec<usize>, n: usize) -> Vec<String> {
        let mut grid: Vec<String> = Vec::with_capacity(n);

        for &col in board {
            let mut row: Vec<char> = vec!['.'; n];
            row[col] = 'Q';
            grid.push(row.iter().collect());
        }

        grid
    }
}

/// Leetcode 51: N-Queens (Bitmask Optimized Version)
///
/// Intuition:
/// ----------
/// Use three bitmasks to track availability of:
/// - columns:         bits set if a column is **occupied**
/// - diagonals1:      \ diagonal conflicts, identified by (row - col + n - 1)
/// - diagonals2:      / diagonal conflicts, identified by (row + col)
///
/// Bitmasking allows constant-time checks and updates using bitwise operators.
///
/// Algorithm:
/// ----------
/// 1. Place one queen per row.
/// 2. For each column, if its bit is not set in any of the masks,
///    it's a valid position.
/// 3. Set the corresponding bits, recurse to the next row,
///    then clear bits to backtrack.
///
/// Time Complexity:
/// ----------------
/// - O(N!) — Same as the standard solution, but bitmasking reduces
///   constant factors significantly.
///
/// Space Complexity:
/// -----------------
/// - O(N^2) for result storage + O(N) recursion depth + O(1) bitmask space
pub struct SolutionAlt;

impl SolutionAlt {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;

        // Stores the final result boards
        let mut results: Vec<Vec<String>> = Vec::new();

        // Used to track column index for placed queens at each row
        let mut queens: Vec<usize> = Vec::with_capacity(n);

        // Start backtracking with all columns and diagonals free (all bits = 0)
        Self::backtrack(
            0,            // current row
            0,            // occupied columns
            0,            // occupied \ diagonals (left)
            0,            // occupied / diagonals (right)
            n,            // board size
            &mut queens,  // queen positions
            &mut results, // final solutions
        );

        results
    }

    /// Recursive bitmask-based backtracking function
    fn backtrack(
        row: usize,                     // current row
        cols: u32,                      // bitmask: occupied columns
        d1: u32,                        // bitmask: \ diagonals
        d2: u32,                        // bitmask: / diagonals
        n: usize,                       // board size
        queens: &mut Vec<usize>,        // queen column positions per row
        results: &mut Vec<Vec<String>>, // final result boards
    ) {
        // Base case: all queens placed
        if row == n {
            results.push(Self::build_board(queens, n));
            return;
        }

        // Compute all free positions for this row
        let mut free: u32 = (!(cols | d1 | d2)) & ((1 << n) - 1);

        // Try each available position
        while free != 0 {
            // Isolate the rightmost free bit
            let bit: u32 = free & (!free + 1);

            // Calculate the column index from the bit
            let col: usize = bit.trailing_zeros() as usize;

            // Place the queen
            queens.push(col);

            // Recurse with updated masks:
            // - cols | bit: mark column as occupied
            // - (d1 | bit) << 1: mark \ diagonal (shift left)
            // - (d2 | bit) >> 1: mark / diagonal (shift right)
            Self::backtrack(
                row + 1,
                cols | bit,
                (d1 | bit) << 1,
                (d2 | bit) >> 1,
                n,
                queens,
                results,
            );

            // Backtrack: remove queen
            queens.pop();

            // Clear the tried position
            free ^= bit;
        }
    }

    /// Converts `queens` vector into board representation
    fn build_board(queens: &Vec<usize>, n: usize) -> Vec<String> {
        let mut board: Vec<String> = Vec::with_capacity(n);

        for &col in queens {
            let mut row = vec!['.'; n];
            row[col] = 'Q';
            board.push(row.iter().collect());
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51_01_n1_single_solution() {
        // For n = 1, only one queen on a 1x1 board
        let result = Solution::solve_n_queens(1);
        let expected = vec![vec!["Q".to_string()]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_51_02_n2_no_solution() {
        // For n = 2, no valid configuration exists
        let result = Solution::solve_n_queens(2);
        let expected: Vec<Vec<String>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_51_03_n3_no_solution() {
        // For n = 3, no valid configuration exists
        let result = Solution::solve_n_queens(3);
        let expected: Vec<Vec<String>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_51_04_n4_two_solutions() {
        // For n = 4, there are exactly 2 valid configurations
        let mut result = Solution::solve_n_queens(4);
        result.sort();
        let mut expected = vec![
            vec![
                ".Q..".to_string(),
                "...Q".to_string(),
                "Q...".to_string(),
                "..Q.".to_string(),
            ],
            vec![
                "..Q.".to_string(),
                "Q...".to_string(),
                "...Q".to_string(),
                ".Q..".to_string(),
            ],
        ];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_51_05_n5_solution_count() {
        // For n = 5, there are 10 valid configurations
        let result = Solution::solve_n_queens(5);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_51_06_n6_solution_count() {
        // For n = 6, there are 4 valid configurations
        let result = Solution::solve_n_queens(6);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_51_07_n7_solution_count() {
        // For n = 7, there are 40 valid configurations
        let result = Solution::solve_n_queens(7);
        assert_eq!(result.len(), 40);
    }

    #[test]
    fn test_51_08_n8_solution_count() {
        // For n = 8, classic case, 92 valid configurations
        let result = Solution::solve_n_queens(8);
        assert_eq!(result.len(), 92);
    }

    #[test]
    fn test_51_09_n9_solution_count() {
        // For n = 9, 352 valid configurations
        let result = Solution::solve_n_queens(9);
        assert_eq!(result.len(), 352);
    }
}
