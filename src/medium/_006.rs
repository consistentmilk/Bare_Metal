//! Intuition:
//! 1. The ZigZag pattern writes characters in a down‑then‑up diagonal across `num_rows` rows.
//! 2. Reading row‑by‑row is equivalent to jumping through the string by alternating step sizes.
//! 3. For each row `i`, the two jumps are:
//!    - `jump_even = 2*(num_rows−1) − 2*i` (vertical‑down to diagonal‑up transition), and
//!    - `jump_odd  = 2*i`            (diagonal‑up back to vertical‑down transition).
//! 4. If either jump is zero (for the first or last row), we substitute the full cycle `2*(num_rows−1)`.
//!
//! Algorithm:
//! 1. If `num_rows == 1`, return `s` unchanged (no ZigZag).  
//! 2. Convert `s` to bytes for O(1) indexing and compute:
//!    - `n = s.len()`  
//!    - `num_rows = num_rows as usize`  
//!    - `cycle = 2*(num_rows−1)`  
//! 3. Allocate `res_stack` with capacity `n` to collect output bytes.  
//! 4. For each row index `r` in `0..num_rows`:
//!    a. Compute `jump_even = cycle − 2*r` and `jump_odd = 2*r`.  
//!    b. If either jump is zero, reset it to `cycle`.  
//!    c. Initialize a cursor `pos = r` and a flag `use_even = true`.  
//!    d. While `pos < n`:
//!       - Push `sbytes[pos]` onto `res_stack`.  
//!       - Advance `pos` by `jump_even` if `use_even`, else by `jump_odd`.  
//!       - Toggle `use_even`.  
//! 5. Convert `res_stack` back to a `String` using `from_utf8_unchecked`.  
//!  
//! Time Complexity: O(n), since each character is visited exactly once.  
//! Space Complexity: O(n), for the output buffer of size `n`.  

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // If there's only one row, the ZigZag is identical to the original string
        if num_rows == 1 {
            return s; // early return for degenerate case
        }

        // Prepare for traversal
        let n: usize = s.len(); // total number of bytes in the string
        let num_rows: usize = num_rows as usize; // convert row count to usize
        let cycle: usize = 2 * (num_rows - 1); // full down‑then‑up cycle length

        let sbytes: &[u8] = s.as_bytes(); // borrow string as byte slice
        let mut res_stack: Vec<u8> = Vec::with_capacity(n); // output buffer for zigzag order

        // Iterate over each row in the ZigZag pattern
        for r in 0..num_rows {
            // Compute the two alternating jump sizes for this row
            let mut jump_even: usize = cycle.saturating_sub(2 * r); // vertical jump
            let mut jump_odd: usize = 2 * r; // diagonal jump

            // If a jump is zero (first or last row), use the full cycle
            if jump_even == 0 {
                jump_even = cycle; // no vertical move → full cycle
            }

            if jump_odd == 0 {
                jump_odd = cycle; // no diagonal move → full cycle
            }

            // Traverse this row by alternating between the two jumps
            let mut pos: usize = r; // starting position in the string
            let mut use_even = true; // start with the "even" jump

            while pos < n {
                res_stack.push(sbytes[pos]); // collect current character

                // Advance by the appropriate jump size
                if use_even {
                    pos += jump_even; // vertical‑down step
                } else {
                    pos += jump_odd; // diagonal‑up step
                }

                use_even = !use_even; // flip for next iteration
            }
        }

        // SAFETY: res_stack contains exactly the original UTF‑8 bytes in new order
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
