/*
Intuition:
1. Build the sequence level by level, adding one new bit at each stage.
2. Reflect the existing codes and prefix them with the new bit to ensure one‑bit transitions.

Algorithm:
1. Start with a list containing only 0.
2. For each bit position i from 0 to n–1:
   a. Compute a mask = 1 << i (sets the iᵗʰ bit).
   b. Iterate over the current list in reverse order.
   c. For each code x in that reverse slice, append (x | mask) to the list.
3. Return the full list of length 2ⁿ.

Time Complexity:
- O(2ⁿ): we generate all 2ⁿ codes, each append is O(1).

Space Complexity:
- O(2ⁿ): the result vector holds 2ⁿ integers.
*/

pub struct Solution;
// Container for the solution method.

impl Solution {
    // Generate n-bit Gray code sequence via reflect-and-prefix using bitwise operations.
    pub fn gray_code(n: i32) -> Vec<i32> {
        // Initialize result vector with capacity 2^n.
        let mut result: Vec<i32> = Vec::with_capacity(1 << n);

        // Seed the sequence with the 0-code.
        result.push(0);

        // For each bit position from 0 to n-1:
        for bit_pos in 0..n {
            // Compute mask that has only the bit_pos bit set.
            let mask: i32 = 1 << (bit_pos as u32);

            // Capture current length before extending.
            let current_len: usize = result.len();

            // Reflect: iterate over existing codes in reverse.
            for idx in (0..current_len).rev() {
                // Append new code by OR’ing the reflected code with mask.
                result.push(result[idx] | mask);
            }
        }

        // Return the completed Gray code sequence.
        result
    }
}

/*
Intuition:
1. Each Gray code value can be obtained by XOR’ing the binary index with itself right‑shifted by one.
2. This ensures only a single bit changes between consecutive values.

Algorithm:
1. Compute the total number of codes as 2ⁿ.
2. Iterate i from 0 up to (but not including) 2ⁿ.
3. Convert each i into its Gray code via the formula, and collect into a Vec.

Time Complexity:
- O(2ⁿ), since we generate and convert each integer exactly once.

Space Complexity:
- O(2ⁿ), to store the resulting sequence of length 2ⁿ.
*/

pub struct SolutionAlt;
// Container for the Gray code method.

impl SolutionAlt {
    // Generate an n‑bit Gray code sequence using the binary‑to‑Gray formula.
    pub fn gray_code(n: i32) -> Vec<i32> {
        // Compute the total number of codes: 2^n.
        let total: i32 = 1 << n;

        // Map each integer in [0, 2^n) to its Gray code and collect into a vector.
        (0..total).map(into_gray).collect()
    }
}

#[inline(always)]
// Convert a binary index into its Gray code equivalent.
fn into_gray(n: i32) -> i32 {
    // XOR n with n shifted right by one bit to flip exactly one bit per increment.
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_089_n1_simple() {
        // Generate 1-bit Gray code sequence
        let result: Vec<i32> = Solution::gray_code(1);

        // Expected sequence for n=1
        let expected: Vec<i32> = vec![0, 1];

        // Assert that the result matches the expected sequence
        assert_eq!(result, expected);
    }

    #[test]
    fn test_089_n2_sequence() {
        // Generate 2-bit Gray code sequence
        let result: Vec<i32> = Solution::gray_code(2);

        // Expected sequence for n=2
        let expected: Vec<i32> = vec![0, 1, 3, 2];

        // Assert that the result matches the expected sequence
        assert_eq!(result, expected);
    }

    #[test]
    fn test_089_n3_sequence() {
        // Generate 3-bit Gray code sequence
        let result: Vec<i32> = Solution::gray_code(3);

        // Expected sequence for n=3
        let expected: Vec<i32> = vec![0, 1, 3, 2, 6, 7, 5, 4];

        // Assert that the result matches the expected sequence
        assert_eq!(result, expected);
    }

    #[test]
    fn test_089_n4_sequence() {
        // Generate 4-bit Gray code sequence
        let result: Vec<i32> = Solution::gray_code(4);

        // Expected sequence for n=4
        let expected: Vec<i32> = vec![0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8];

        // Assert that the result matches the expected sequence
        assert_eq!(result, expected);
    }

    #[test]
    fn test_089_n5_length_and_wraparound() {
        // Generate 5-bit Gray code sequence
        let result: Vec<i32> = Solution::gray_code(5);

        // Verify the sequence has length 32
        assert_eq!(result.len(), 32);

        // Verify the sequence starts with 0
        assert_eq!(result[0], 0);

        // Compute the XOR of the first and last elements
        let wrap_xor: i32 = result[0] ^ result[31];

        // Verify that the Hamming distance is exactly one bit
        assert_eq!(wrap_xor.count_ones(), 1);
    }
}
