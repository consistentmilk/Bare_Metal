//! # Elimination Game
//!
//! ## Intuition
//! We simulate the elimination by tracking only four variables:
//! - `head`: the first remaining number in the virtual list.
//! - `step`: the gap between consecutive survivors.
//! - `remaining`: how many numbers are still in play.
//! - `left_to_right`: direction of elimination (true = left→right).
//! Each pass halves the list; depending on direction and parity, `head` may shift by `step`.
//!
//! ## Algorithm
//! 1. Initialize:
//!    - `head = 1`  
//!    - `step = 1`  
//!    - `remaining = n`  
//!    - `left_to_right = true`  
//! 2. While `remaining > 1`:
//!    - If `left_to_right`, do `head += step`.
//!    - Else if `remaining` is odd, do `head += step`.
//!    - Then `remaining /= 2`, `step *= 2`, and flip `left_to_right`.
//! 3. Return `head`.
//!
//! ## Complexity
//! - Time: O(log n), since each iteration halves `remaining`.  
//! - Space: O(1), only a fixed number of variables.

pub struct Solution;

impl Solution {
    /// Returns the last remaining number after repeatedly removing every second element.
    ///
    /// # Arguments
    ///
    /// * `n` - The size of the initial list [1, 2, ..., n].
    ///
    /// # Returns
    ///
    /// * `i32` - The last number remaining.
    #[inline]
    pub fn last_remaining(n: i32) -> i32 {
        // The current first element in the list.
        let mut head: i32 = 1;

        // The interval between consecutive survivors.
        let mut step: i32 = 1;

        // How many elements remain.
        let mut remaining: i32 = n;

        // Direction flag: true = left→right, false = right→left.
        let mut left_to_right: bool = true;

        // Continue eliminating until one element is left.
        while remaining > 1 {
            // On a left→right pass, the head always moves forward by `step`.
            if left_to_right {
                head = head + step;
            }
            // On a right→left pass with odd count, the head also moves forward.
            else if remaining & 1 == 1 {
                head = head + step;
            }

            // Halve the number of elements (we remove every other one).
            remaining = remaining / 2;

            // Double the gap between survivors for the next round.
            step = step * 2;

            // Switch elimination direction.
            left_to_right = !left_to_right;
        }

        // The last remaining number.
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_390_single_element() {
        assert_eq!(Solution::last_remaining(1), 1);
    }

    #[test]
    fn test_390_two_elements() {
        assert_eq!(Solution::last_remaining(2), 2);
    }

    #[test]
    fn test_390_three_elements() {
        assert_eq!(Solution::last_remaining(3), 2);
    }

    #[test]
    fn test_390_four_elements() {
        assert_eq!(Solution::last_remaining(4), 2);
    }

    #[test]
    fn test_390_five_elements() {
        assert_eq!(Solution::last_remaining(5), 2);
    }

    #[test]
    fn test_390_six_elements() {
        assert_eq!(Solution::last_remaining(6), 4);
    }

    #[test]
    fn test_390_seven_elements() {
        assert_eq!(Solution::last_remaining(7), 4);
    }

    #[test]
    fn test_390_eight_elements() {
        assert_eq!(Solution::last_remaining(8), 6);
    }

    #[test]
    fn test_390_nine_elements() {
        assert_eq!(Solution::last_remaining(9), 6);
    }

    #[test]
    fn test_390_ten_elements() {
        assert_eq!(Solution::last_remaining(10), 8);
    }
}
