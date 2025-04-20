//! Problem: LeetCode 389 - Find the Difference
//!
//! Intuition:
//! We're given two strings: `s` and `t`, where `t` is a shuffled version of `s` with one extra character added.
//! The goal is to find the extra character efficiently.
//!
//! We can solve this using XOR logic: since `a ^ a = 0` and `a ^ 0 = a`, XORing all characters in `s` and `t`
//! will cancel out all characters that appear in both and leave the one that appears only in `t`.
//!
//! This is constant space and highly performant. We use `char::from_u32_unchecked` because the result
//! will always be a valid ASCII character due to the problem constraints.
//!
//! Algorithm:
//! 1. Initialize an accumulator variable `acc = 0`.
//! 2. XOR all characters in `t` into `acc`.
//! 3. XOR all characters in `s` into `acc`.
//! 4. The result will be the extra character in `t`.
//!
//! Time Complexity: O(n)
//! Space Complexity: O(1)

pub struct Solution;

impl Solution {
    /// Finds the extra character in the string `t` not found in `s`.
    ///
    /// # Arguments
    /// * `s` - Original string
    /// * `t` - Modified string with one additional character
    ///
    /// # Returns
    /// * `char` - The extra character
    #[inline]
    pub fn find_the_difference(s: String, t: String) -> char {
        // Initialize the XOR accumulator
        let mut acc: u32 = 0;

        // XOR all characters from string `t`
        for ch in t.chars() {
            acc ^= ch as u32;
        }

        // XOR all characters from string `s`
        for ch in s.chars() {
            acc ^= ch as u32;
        }

        // Convert final result to character (safe due to ASCII range)
        unsafe { char::from_u32_unchecked(acc) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_389_1_extra_at_end() {
        let s = "abcd".to_string();
        let t = "abcde".to_string();
        assert_eq!(Solution::find_the_difference(s, t), 'e');
    }

    #[test]
    fn test_389_2_extra_at_start() {
        let s = "xyz".to_string();
        let t = "wxyz".to_string();
        assert_eq!(Solution::find_the_difference(s, t), 'w');
    }

    #[test]
    fn test_389_3_single_char() {
        let s = "".to_string();
        let t = "x".to_string();
        assert_eq!(Solution::find_the_difference(s, t), 'x');
    }

    #[test]
    fn test_389_4_duplicate_chars() {
        let s = "aabbcc".to_string();
        let t = "aabbcce".to_string();
        assert_eq!(Solution::find_the_difference(s, t), 'e');
    }

    #[test]
    fn test_389_5_unicode_ascii_digits() {
        let s = "1234".to_string();
        let t = "41235".to_string();
        assert_eq!(Solution::find_the_difference(s, t), '5');
    }

    #[test]
    fn test_389_6_extra_middle() {
        let s = "rstuv".to_string();
        let t = "trsuvx".to_string();
        assert_eq!(Solution::find_the_difference(s, t), 'x');
    }

    #[test]
    fn test_389_7_last_in_order() {
        let s = "mnopqr".to_string();
        let t = "mnopqrs".to_string();
        assert_eq!(Solution::find_the_difference(s, t), 's');
    }

    #[test]
    fn test_389_8_empty_input() {
        let s = "".to_string();
        let t = "a".to_string();
        assert_eq!(Solution::find_the_difference(s, t), 'a');
    }

    #[test]
    fn test_389_9_long_input() {
        let s = "a".repeat(100_000);
        let mut t = s.clone();
        t.push('z');
        assert_eq!(Solution::find_the_difference(s, t), 'z');
    }

    #[test]
    fn test_389_10_case_sensitive_valid_extra() {
        let s = "abcde".to_string();
        let t = "abcdeE".to_string(); // now has extra character 'E'
        assert_eq!(Solution::find_the_difference(s, t), 'E');
    }
}
