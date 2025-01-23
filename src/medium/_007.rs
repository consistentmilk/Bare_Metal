pub struct Solution;

impl Solution {
    /// Reverses the given integer `x`.
    ///
    /// This function takes an integer `x` and returns its reversed form.
    /// It handles both positive and negative numbers and accounts for overflow.
    ///
    /// # Arguments
    ///
    /// * `x` - The integer to be reversed.
    ///
    /// # Returns
    ///
    /// * The reversed form of the integer `x`.
    /// * If an overflow occurs, it returns 0.
    pub fn reverse(x: i32) -> i32 {
        // Initialize the reversed integer to 0 and a boolean variable to track if `x` is negative.
        let mut rev: i32 = 0;
        let (mut num, is_negative) = if x < 0 { (-x, true) } else { (x, false) };

        // Iterate through each digit of `num` from the right.
        while num > 0 {
            // Get the rightmost digit of `num`.
            let digit: i32 = num % 10;

            // Multiply `rev` by 10 and add the current digit.
            // This operation can fail due to overflow, so we use
            // `checked_mul` and `checked_add`.
            if let Some(new_rev) = rev.checked_mul(10).and_then(|r| r.checked_add(digit)) {
                rev = new_rev;
            } else {
                // If an overflow occur, return 0.
                return 0;
            }

            // Remove the rightmost digit from `num`.
            num /= 10;
        }

        // If `x` was negative, negate `rev`.
        if is_negative {
            if let Some(new_rev) = rev.checked_neg() {
                rev = new_rev;
            } else {
                // If an overflow occur, return 0.
                return 0;
            }
        }

        // Return the reversed form of `x`.
        rev
    }

    /// A variation of the `reverse` function that uses bitwise operations to
    /// handle overflow and does not use the `checked_mul` and `checked_add`
    /// functions.
    ///
    /// # Arguments
    ///
    /// * `x` - The integer to be reversed.
    ///
    /// # Returns
    ///
    /// * The reversed form of the integer `x`.
    /// * If an overflow occur, it returns 0.
    pub fn reverse_alt(mut x: i32) -> i32 {
        // Check if `x` is the minimum possible integer, in which case
        // the reversal would result in an overflow and we return 0.
        if x == i32::MIN {
            return 0;
        }

        // Check if `x` is negative and store the result in a boolean variable.
        let is_negative: bool = x < 0;

        // Take the absolute value of `x` to handle the negative sign.
        x = x.wrapping_abs();

        // Initialize the reversed integer to 0 and a mutable variable to
        // store the number to be reversed.
        let mut rev: i32 = 0;
        let mut num: i32 = x;

        // Iterate through each digit of `num` from the right.
        while num != 0 {
            // Get the rightmost digit of `num`.
            let digit: i32 = num % 10;

            // Multiply `rev` by 10 and add the current digit.
            rev = rev.wrapping_mul(10).wrapping_add(digit);

            // Remove the rightmost digit from `num`.
            num /= 10;
        }

        // If `x` was negative, negate `rev`.
        if is_negative {
            rev = rev.wrapping_neg();
        }

        // Return the reversed form of `x`.
        rev
    }
}
