pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let is_neg: bool = (dividend < 0) ^ (divisor < 0);

        let mut p: i32 = if dividend > 0 { -dividend } else { dividend };
        let q: i32 = if divisor > 0 { -divisor } else { divisor };

        // res is the *negative* absolute value of the quotient, in case the quotient is i32::MIN.
        let mut res: i32 = 0;

        // Here is the magic: the negative divisor(aka q) can left-shift atmost q.leading_ones() - 1 bits before it overflows.
        for shift in (0..q.leading_ones()).rev() {
            // Don't apply "p -= (q << shift)" before overflow checking. Consider p = 0, q << shift = i32::MIN.
            if p <= (q << shift) {
                p -= q << shift;
                res += -1 << shift; // Not "res -= 1 << shift;" which may cause overflow.
            }
        }

        if is_neg {
            res
        } else if res == i32::MIN {
            i32::MAX
        } else {
            -res
        }
    }
}
