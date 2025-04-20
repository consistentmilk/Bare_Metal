//! ## Intuition
//! We use dynamic programming to decide at each question whether to solve it (gaining points and
//! skipping a certain number of subsequent questions based on brainpower) or skip it, maximizing
//! total points.
//!
//! ## Algorithm
//! 1. Let `n` be the number of questions.
//! 2. Create a DP array `dp` of length `n + 1`, where `dp[i]` represents the maximum points achievable
//!    from question `i` onward.
//! 3. Iterate `i` from `n - 1` down to `0`:
//!    - Let `point` be the points for question `i`.
//!    - Calculate `skip = min(n, i + 1 + brainpower_i)` to find the next question index if solved.
//!    - Set `dp[i] = max(point + dp[skip], dp[i + 1])`.
//! 4. Return `dp[0]`.
//!
//! ## Time Complexity
//! O(n), where n = number of questions.
//!
//! ## Space Complexity
//! O(n) for the DP array.
pub struct Solution;

impl Solution {
    /// Calculates the maximum total points achievable by optimally solving or skipping exam questions.
    ///
    /// # Arguments
    ///
    /// * `questions` - A `Vec<Vec<i32>>` where each inner vector has two elements:
    ///     - `[0]`: the points awarded for solving question `i`.
    ///     - `[1]`: the brainpower required, i.e. the number of subsequent questions to skip if solved.
    ///
    /// # Returns
    ///
    /// * `i64` - The maximum total points one can earn starting from question 0 through question `n-1`.
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        // Number of questions.
        let n: usize = questions.len();

        // DP array of size n + 1, initialized to zero.
        let mut dp: Vec<i64> = vec![0; n + 1];

        // Build the DP table from the end towards the start.
        for i in (0..n).rev() {
            // Points awarded for solving question i.
            let point: i64 = questions[i][0] as i64;

            // Compute the index to jump to after solving question i.
            let mut skip: usize = i + 1 + questions[i][1] as usize;

            // Cap skip so it does not exceed the total number of questions.
            skip = std::cmp::min(n, skip);

            // Option 1: Solve question i and add dp[skip].
            // Option 2: Skip question i and take dp[i + 1].
            dp[i] = std::cmp::max(point + dp[skip], dp[i + 1]);
        }

        // Maximum points achievable starting from question 0.
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2140_basic_example() {
        // Basic example from problem statement
        let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        let expected: i64 = 5;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_increasing_brainpower() {
        // Increasing points and brainpower equal to index
        let questions = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        let expected: i64 = 7;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_single_question() {
        // Single question should take its points
        let questions = vec![vec![10, 0]];
        let expected: i64 = 10;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_two_questions_no_skip() {
        // Two questions without needing to skip
        let questions = vec![vec![5, 0], vec![7, 0]];
        let expected: i64 = 12;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_two_questions_with_skip() {
        // First question forces skip over second
        let questions = vec![vec![5, 1], vec![7, 0]];
        let expected: i64 = 7;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_all_zero_points() {
        // All questions yield zero points
        let questions = vec![vec![0, 0], vec![0, 0], vec![0, 0]];
        let expected: i64 = 0;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_alternating_skips() {
        // Alternating skip pattern example
        let questions = vec![vec![3, 1], vec![2, 1], vec![10, 1], vec![5, 1]];
        let expected: i64 = 13;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_large_brainpower() {
        // Brainpower exceeds remaining questions
        let questions = vec![vec![5, 5], vec![1, 1], vec![1, 1]];
        let expected: i64 = 5;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_complex_pattern() {
        // Complex mixed pattern
        let questions = vec![vec![4, 2], vec![1, 2], vec![2, 1], vec![7, 3], vec![3, 1]];
        let expected: i64 = 11;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_random_pattern() {
        // Random small pattern
        let questions = vec![vec![2, 1], vec![1, 0], vec![4, 1], vec![6, 0]];
        let expected: i64 = 8;
        let result: i64 = Solution::most_points(questions);
        assert_eq!(result, expected);
    }
}
