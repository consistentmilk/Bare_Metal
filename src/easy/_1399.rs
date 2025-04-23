/*
============================================================================
LeetCode 1399 ­— Count Largest Group
============================================================================
Intuition
---------
• Partition the integers 1..=n by the sum of their decimal digits.
• If digit-sum s appears kₛ times, let max = maxₛ kₛ.
• Return the number of distinct s whose bucket size equals max.

Algorithm
---------
1. Create a fixed-length array `cnt[37]` (index 0 unused):
      digit sums for numbers ≤ 9999 lie in 1..=36.
2. For each x ∈ 1..=n
      a. Compute `s = digit_sum(x)` in O(#digits).
      b. Increment `cnt[s]`.
3. Scan `cnt` once to find `max_size`.
4. Scan again to count how many bins equal `max_size`.
5. Return that count.

Time Complexity
---------------
O(n · d) where d ≤ 4 (digits of 10 000) ⇒ O(n).

Space Complexity
----------------
O(1) — the counter array has constant length 37.

============================================================================
*/

pub struct Solution;

impl Solution {
    //───────────────────────────────────────────────────────────────────────
    // Compute the digit-sum of a non-negative integer in base-10.
    //───────────────────────────────────────────────────────────────────────
    fn digit_sum(mut x: i32) -> usize {
        //·Initialize running sum.
        let mut sum: usize = 0;

        //·Extract decimal digits until x becomes 0.
        while x != 0 {
            //·Add least-significant digit to the sum.
            sum += (x % 10) as usize;

            //·Discard the processed digit.
            x /= 10;
        }

        sum
    }

    //───────────────────────────────────────────────────────────────────────
    // Public interface
    //───────────────────────────────────────────────────────────────────────
    pub fn count_largest_group(n: i32) -> i32 {
        //·Fixed counter array: indices 1..=36 hold bucket sizes.
        let mut cnt: [i32; 37] = [0; 37];

        //·Populate counters for every integer in 1..=n.
        for x in 1..=n {
            //·Compute digit sum (max 36).
            let s: usize = Self::digit_sum(x);

            //·Increment the appropriate bucket.
            cnt[s] += 1;
        }

        //·Determine the maximum bucket size.
        let mut max_size: i32 = 0;

        for &c in cnt.iter().skip(1) {
            if c > max_size {
                max_size = c;
            }
        }

        //·Count how many buckets attain this maximum size.
        let mut groups: i32 = 0;

        for &c in cnt.iter().skip(1) {
            if c == max_size {
                groups += 1;
            }
        }

        groups
    }
}

/*
===========================================================================
 LeetCode 1399 — Count Largest Group
 Optimised version (branch-free digit sums, O(1) memory)
===========================================================================

Intuition
---------
* We only need the **digit-sum** σ(i) for every i ∈ [1, n].
* n ≤ 10 000 → σ(i) ∈ [1, 36].  We therefore keep a 37-element counter.
* To avoid per-number division loops, pre-compute σ(i) in one pass:

        σ(i) = σ(i / 10) + (i % 10)

  Hence each σ(i) re-uses σ(i/10) already computed.

Algorithm
---------
1. Allocate `cnt[37]` = 0.
2. Initialise `digit_sum[0] = 0`.
3. For i = 1 … n
      a. σ(i) = σ(i/10) + i%10   // one % and one table lookup
      b. cnt[σ(i)] += 1
4. Scan `cnt` to find `max_size`.
5. Scan again to count how many bins equal `max_size`.
6. Return that count.

Time Complexity
---------------
O(n) with **one** `% 10` and **no branches** per integer.

Space Complexity
----------------
cnt[37]  +  digit_sum array ≤ 10 001 bytes → O(1).

===========================================================================
*/

pub struct SolutionOpt;

impl SolutionOpt {
    //──────────────────────────────────────────────────────────────────────
    // Main entry point
    //──────────────────────────────────────────────────────────────────────
    pub fn count_largest_group(n: i32) -> i32 {
        // Counter: index = digit-sum (1..=36), value = bucket size.
        let mut cnt: [i32; 37] = [0; 37];

        // Pre-compute digit sums up to n on the fly.
        // digit_sum[i] fits in u8 (≤ 36).  We reuse the same Vec to
        // avoid a second pass.
        let mut digit_sum: Vec<u8> = vec![0; n as usize + 1];

        // i = 1 … n
        for i in 1..=n as usize {
            // σ(i) = σ(i / 10) + (i % 10)  (branch-free)
            let sigma: u8 = digit_sum[i / 10] + (i % 10) as u8;

            digit_sum[i] = sigma; // store for reuse
            cnt[sigma as usize] += 1; // update bucket
        }

        // Determine the maximum bucket size.
        let mut max_size: i32 = 0;
        for &c in &cnt[1..=36] {
            if c > max_size {
                max_size = c;
            }
        }

        // Count how many buckets reach that maximum.
        let mut groups = 0;
        
        for &c in &cnt[1..=36] {
            if c == max_size {
                groups += 1;
            }
        }

        groups
    }
}

/*─────────────────────────────  Unit Tests  ─────────────────────────────*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1399_example_1() {
        //·n = 13 ⟹ answer = 4
        let expected: i32 = 4;
        assert_eq!(Solution::count_largest_group(13), expected);
    }

    #[test]
    fn test_1399_example_2() {
        //·n = 2 ⟹ answer = 2
        let expected: i32 = 2;
        assert_eq!(Solution::count_largest_group(2), expected);
    }

    #[test]
    fn test_1399_single_number() {
        //·n = 1 ⟹ only one group
        assert_eq!(Solution::count_largest_group(1), 1);
    }

    #[test]
    fn test_1399_upper_bound() {
        //·n = 10 000 (problem constraint upper bound)
        //·Sanity check: result should be > 0.
        assert!(Solution::count_largest_group(10_000) > 0);
    }
}
