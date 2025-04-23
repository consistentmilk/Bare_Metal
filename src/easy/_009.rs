/*
Intuition:
  Negative numbers and numbers ending with zero (except zero)
  cannot be palindromes. Reverse half the digits and compare.

Algorithm:
1. Return false for x < 0 or (x % 10 == 0 && x != 0).
2. Initialize orig = x and rev_half = 0.
3. While orig > rev_half:
   a. rev_half = rev_half * 10 + orig % 10.
   b. orig /= 10.
4. Return orig == rev_half or orig == rev_half / 10.

Time Complexity: O(log n), where n is x.
Processes half the digits, so still O(log n).
Space Complexity: O(1).
*/

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Return false for negatives or trailing zeros (except zero).
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        // orig: remaining part of the number.
        let mut orig: i32 = x;
        // rev_half: reversed digits of the latter half.
        let mut rev_half: i32 = 0;

        // Reverse digits until orig <= rev_half.
        while orig > rev_half {
            // Move last digit from orig to rev_half.
            rev_half = rev_half * 10 + orig % 10;

            // Remove last digit from orig.
            orig /= 10;
        }

        // For even digit count: orig == rev_half.
        // For odd digit count: orig == rev_half / 10.
        orig == rev_half || orig == rev_half / 10
    }

    pub fn is_palindrome_naive(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut x_tmp: i32 = x;
        let mut x_rev: i32 = 0;

        while x_tmp > 0 {
            x_rev = (x_rev * 10) + (x_tmp % 10);

            x_tmp = x_tmp / 10;
        }

        x == x_rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_009_1() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn test_009_2() {
        assert!(!Solution::is_palindrome(123));
    }

    #[test]
    fn test_009_3() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn test_009_4() {
        assert!(Solution::is_palindrome(1));
    }
}
