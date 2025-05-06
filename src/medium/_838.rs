/*
LeetCode 838 – Push Dominoes

INTUITION
------------------------------------------------------------------
Interpret each non‑dot as an "anchor" exerting force outward.
A dot block is affected only by its bordering anchors,
so we can process blocks independently in a single sweep.

ALGORITHM
------------------------------------------------------------------
1. Convert input &str into a mutable Vec<u8> (`buf`).
2. Initialise
      left_idx  = -1               // virtual anchor before start
      left_sym  = b'L'             // acts like infinite 'L'
3. Iterate i from 0..=n:
      • If i == n treat sym = b'R' // virtual anchor after end
      • Else sym = buf[i]
      • When sym != b'.' OR i == n:
           right_idx = i
           right_sym = sym
           fill_segment(buf, left_idx, left_sym,
                             right_idx, right_sym)
           left_idx = right_idx
           left_sym = right_sym
4. Convert buf back into String and return.

fill_segment handles the four cases in O(length) with direct
writes into buf; no extra allocations.

TIME COMPLEXITY
------------------------------------------------------------------
O(n) – each domino visited constant times.

SPACE COMPLEXITY
------------------------------------------------------------------
O(1) auxiliary; the mutable buffer reuses the input size.
*/

pub struct Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        // --- convert to mutable byte buffer -----------------------
        let mut buf: Vec<u8> = dominoes.into_bytes();
        let n: usize = buf.len();

        // --- anchor state (use isize for -1 sentinel) -------------
        let mut left_idx: isize = -1; // index of last anchor
        let mut left_sym: u8 = b'L'; // symbol of last anchor

        // --- iterate through positions plus sentinel --------------
        for i in 0..=n {
            // obtain current symbol (b'.' if middle dots,
            // b'R' sentinel when i == n)
            let sym: u8 = if i == n { b'R' } else { buf[i] };

            if sym != b'.' || i == n {
                // current index becomes right anchor
                let right_idx: isize = i as isize;
                let right_sym: u8 = sym;

                // process the open segment (left_idx, right_idx)
                Self::fill_segment(&mut buf, left_idx, left_sym, right_idx, right_sym);

                // move right anchor to be next left anchor
                left_idx = right_idx;
                left_sym = right_sym;
            }
        }

        // --- convert back to String -------------------------------
        unsafe { String::from_utf8_unchecked(buf) }
    }

    // --------------------------------------------------------------
    // Fill dots between two anchors according to the pattern
    // --------------------------------------------------------------
    fn fill_segment(
        buf: &mut [u8],
        left_idx: isize,
        left_sym: u8,
        right_idx: isize,
        right_sym: u8,
    ) {
        // distance between anchors
        let mut l: isize = left_idx + 1;
        let mut r: isize = right_idx - 1;

        match (left_sym, right_sym) {
            (b'L', b'L') => {
                // case: L … L  -> all fall left
                while l <= r {
                    buf[l as usize] = b'L';
                    l += 1;
                }
            }

            (b'R', b'R') => {
                // case: R … R  -> all fall right
                while l <= r {
                    buf[l as usize] = b'R';
                    l += 1;
                }
            }

            (b'L', b'R') => {
                // case: L … R  -> forces diverge; do nothing
            }

            (b'R', b'L') => {
                // case: R … L  -> forces converge
                while l < r {
                    buf[l as usize] = b'R'; // left side falls right
                    buf[r as usize] = b'L'; // right side falls left
                    l += 1;
                    r -= 1;
                }

                // middle domino (if any) remains '.'
            }

            _ => unreachable!(), // sentinels chosen to avoid this
        }
    }
}

// ================================================================
// Unit‑tests for LeetCode 838 – Push Dominoes
// ================================================================
#[cfg(test)]
mod tests {
    // Bring the solution into scope
    use super::Solution;

    // ------------------------------------------------------------
    // 01 – Official sample 1 (“RR.L” ➜ “RR.L”)
    // ------------------------------------------------------------
    #[test]
    fn test_838_example1_01() {
        // construct the first sample input
        let input = "RR.L".to_string();
        // expected final state for sample 1
        let expected = "RR.L".to_string();
        // verify the solver output equals expectation
        assert_eq!(Solution::push_dominoes(input), expected,);
    }

    // ------------------------------------------------------------
    // 02 – Official sample 2
    //      (“.L.R...LR..L..” ➜ “LL.RR.LLRRLL..”)
    // ------------------------------------------------------------
    #[test]
    fn test_838_example2_02() {
        // construct the second sample input
        let input = ".L.R...LR..L..".to_string();
        // expected final state for sample 2
        let expected = "LL.RR.LLRRLL..".to_string();
        // perform the assertion
        assert_eq!(Solution::push_dominoes(input), expected,);
    }

    // ------------------------------------------------------------
    // 03 – Edge: all dominoes untouched (“....” ➜ “....”)
    // ------------------------------------------------------------
    #[test]
    fn test_838_all_dots_03() {
        // input with no initial pushes
        let input = "....".to_string();
        // same string should be returned
        let expected = "....".to_string();
        // check solver behaviour on neutral case
        assert_eq!(Solution::push_dominoes(input), expected,);
    }

    // ------------------------------------------------------------
    // 04 – Edge: single left push at start (“L....” ➜ “L....”)
    // ------------------------------------------------------------
    #[test]
    fn test_838_single_left_04() {
        // leftmost domino pushed left
        let input = "L....".to_string();
        // entire row must fall left
        let expected = "L....".to_string();
        // assertion for left‑propagation
        assert_eq!(Solution::push_dominoes(input), expected,);
    }

    // ------------------------------------------------------------
    // 05 – Edge: single right push at end (“....R” ➜ “....R”)
    // ------------------------------------------------------------
    #[test]
    fn test_838_single_right_05() {
        // rightmost domino pushed right
        let input = "....R".to_string();
        // entire row must fall right
        let expected = "....R".to_string();
        // assertion for right‑propagation
        assert_eq!(Solution::push_dominoes(input), expected,);
    }

    // ------------------------------------------------------------
    // 06 – Balanced hits (“R.L” ➜ “R.L”)
    //      Forces cancel at the centre
    // ------------------------------------------------------------
    #[test]
    fn test_838_balanced_06() {
        // opposing pushes meet with no gap
        let input = "R.L".to_string();
        // middle domino stays upright
        let expected = "R.L".to_string();
        // check correct cancellation
        assert_eq!(Solution::push_dominoes(input), expected,);
    }

    // ------------------------------------------------------------
    // 07 – Converging even gap (“R..L” ➜ “RRLL”)
    //      Two dots between R and L (even distance)
    // ------------------------------------------------------------
    #[test]
    fn test_838_even_converge_07() {
        // right‑then‑left with two neutral dots
        let input = "R..L".to_string();
        // both dots should fall towards their nearest force
        let expected = "RRLL".to_string();
        // assert even‑gap convergence handling
        assert_eq!(Solution::push_dominoes(input), expected,);
    }
}
