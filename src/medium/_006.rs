//! # Zigzag Conversion (LeetCode 6)
//!
//! ## Intuition
//! 1. Characters are placed in a down‑then‑up diagonal across `num_rows` rows.
//! 2. Reading row by row corresponds to jumping through the string with alternating steps.
//! 3. For row `r`, the jumps are:
//!    - `jump_even = cycle - 2 * r`
//!    - `jump_odd  = 2 * r`
//! 4. If a jump is zero, replace it with `cycle`.
//!
//! ## Algorithm
//! 1. Handle degenerate case `num_rows == 1` (return `s`).  
//! 2. Compute:
//!    - `n = s.len()`  
//!    - `num_rows = num_rows as usize`  
//!    - `cycle = 2 * (num_rows - 1)`  
//! 3. Convert `s` to bytes and prepare an output buffer of capacity `n`.  
//! 4. For each row `r` in `0..num_rows`:
//!    a. Compute `jump_even = cycle - 2 * r` and `jump_odd = 2 * r`, substituting `cycle` if zero.  
//!    b. Initialize `pos = r` and `use_even = true`.  
//!    c. While `pos < n`:
//!       - Append `sbytes[pos]` to the buffer.  
//!       - Advance `pos` by `jump_even` if `use_even`, else by `jump_odd`.  
//!       - Toggle `use_even`.  
//! 5. Convert buffer back to `String` with `from_utf8_unchecked`.
//!
//! ## Time Complexity
//! O(n), each character visited once.
//!
//! ## Space Complexity
//! O(n), for the output buffer.

pub struct Solution;

impl Solution {
    /// Converts a string into its zigzag pattern and reads it line by line.
    ///
    /// # Arguments
    ///
    /// * `s` – The input string to convert.
    /// * `num_rows` – The number of rows in the zigzag pattern.
    ///
    /// # Returns
    ///
    /// * `String` – The zigzag‑converted string.
    #[inline]
    pub fn convert(s: String, num_rows: i32) -> String {
        // If only one row, the zigzag pattern is identical to the original string.
        if num_rows == 1 {
            return s;
        }

        // Determine the total number of bytes in the string.
        let n: usize = s.len();

        // Convert the row count from i32 to usize for indexing.
        let num_rows: usize = num_rows as usize;

        // Compute the length of one full zigzag cycle (down then up).
        let cycle: usize = 2 * (num_rows - 1);

        // Borrow the string as a slice of bytes for O(1) indexing.
        let sbytes: &[u8] = s.as_bytes();

        // Prepare a buffer to collect the resulting characters in order.
        let mut res_stack: Vec<u8> = Vec::with_capacity(n);

        // Process each row of the zigzag pattern.
        for r in 0..num_rows {
            // Compute the jump length when moving vertically down.
            let mut jump_even: usize = cycle.saturating_sub(2 * r);

            // Compute the jump length when moving diagonally up.
            let mut jump_odd: usize = 2 * r;

            // If vertical jump is zero (first or last row), use the full cycle.
            if jump_even == 0 {
                jump_even = cycle;
            }

            // If diagonal jump is zero (first or last row), use the full cycle.
            if jump_odd == 0 {
                jump_odd = cycle;
            }

            // Initialize position to the start of this row.
            let mut pos: usize = r;

            // Flag indicating which jump to use next (start with even).
            let mut use_even: bool = true;

            // Traverse positions for this row until the end of the string.
            while pos < n {
                // Collect the character at the current position.
                res_stack.push(sbytes[pos]);

                // Advance position by the appropriate jump length.
                if use_even {
                    pos += jump_even;
                } else {
                    pos += jump_odd;
                }

                // Toggle between even and odd jumps for next step.
                use_even = !use_even;
            }
        }

        // SAFETY: res_stack contains all original bytes in valid UTF‑8 order.
        unsafe { String::from_utf8_unchecked(res_stack) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_006_1_example_three_rows() {
        let test_str = "PAYPALISHIRING".to_string();
        let test_num_rows = 3;
        let expected = "PAHNAPLSIIGYIR".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_006_2_example_four_rows() {
        let test_str = "PAYPALISHIRING".to_string();
        let test_num_rows = 4;
        let expected = "PINALSIGYAHRPI".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_006_3_empty_string() {
        let test_str = "".to_string();
        let test_num_rows = 3;
        let expected = "".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_006_4_single_character() {
        let test_str = "A".to_string();
        let test_num_rows = 2;
        let expected = "A".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_006_5_one_row_no_zigzag() {
        let test_str = "AB".to_string();
        let test_num_rows = 1;
        let expected = "AB".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_006_6_two_rows_abcd() {
        let test_str = "ABCD".to_string();
        let test_num_rows = 2;
        let expected = "ACBD".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_006_7_two_rows_abcde() {
        let test_str = "ABCDE".to_string();
        let test_num_rows = 2;
        let expected = "ACEBD".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_006_8_three_rows_abcde() {
        let test_str = "ABCDE".to_string();
        let test_num_rows = 3;
        let expected = "AEBDC".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_006_9_no_transform_one_row() {
        let test_str = "PAYPALISHIRING".to_string();
        let test_num_rows = 1;
        let expected = "PAYPALISHIRING".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_006_10_rows_exceed_length() {
        let test_str = "SHORT".to_string();
        let test_num_rows = 10;
        let expected = "SHORT".to_string();
        assert_eq!(Solution::convert(test_str, test_num_rows), expected);
    }
}
