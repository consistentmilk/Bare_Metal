pub struct Solution {}

impl Solution {
    pub fn is_palindrome_bounds_checked(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut x_tmp: i32 = x;
        let mut x_rev: i32 = 0;

        while x_tmp > 0 {
            let digit: i32 = x_tmp % 10;

            if let Some(val_1) = x_rev.checked_mul(10) {
                if let Some(val_2) = val_1.checked_add(digit) {
                    x_rev = val_2;
                } else {
                    return false;
                }
            } else {
                return false;
            }

            x_tmp = x_tmp / 10;
        }

        x == x_rev
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut x_tmp: i32 = x;
        let mut x_rev: i32 = 0;

        while x_tmp > 0 {
            x_rev = x_rev * 10 + (x_tmp % 10);
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
