/**
Intuition
---------
This problem requires simulating pattern matching with support for two
special characters:
- '.' matches any character
- '*' matches zero or more of the preceding character

We use dynamic programming to evaluate match validity incrementally. A
space-optimized solution only needs the previous row of the DP table, so
we use a 1D DP array. Each entry dp[j] stores whether s[0..i] matches
p[0..j].

We track:
- prev: represents the "diagonal" value (dp[i-1][j-1])
- temp: stores dp[j] before it’s updated, for safe shifting

Matching with '.' and '*' can be solved by DP.  A 2-D table costs
O(m · n) space; collapsing to one row reduces space to O(n).  We still
run O(m · n) time, but careful memory reuse plus early pruning gives
close-to-linear behaviour on many practical inputs.

Algorithm
---------
1. Build `star_of[j] = true` if `p[j] == '*'` (single scan).
2. Initialise a 1-D `dp` where `dp[j]` ⇒ s[0..i] matches p[0..j].
3. Pre-fill `dp` for an empty string using the rule `"x*"` → "".
4. For each byte `s[i-1]`:
   a. Reset `dp[0] = false` (non-empty s cannot match empty pattern).
   b. Iterate `j = 1..n`:
      · If `p[j-1] != '*'`
          `dp[j] = prev && char_eq`
      · Else (`'*'`)
          `dp[j] = dp[j-2] || (dp[j] && char_eq_prev)`
      Only two temporaries (`prev`, `temp`) are needed.
5. Early break if `dp_rest_possible` is false (pruning).
6. Return `dp[n]`.

Time Complexity   O(m · n) worst-case
Space Complexity  O(n)

Notation
--------
char_eq      = p[j-1] == '.' | p[j-1] == s[i-1]
char_eq_prev = p[j-2] == '.' | p[j-2] == s[i-1]
*/
pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // Convert to byte slices for cheap indexing
        let sb: &[u8] = s.as_bytes();
        let pb: &[u8] = p.as_bytes();
        let m: usize = sb.len();
        let n: usize = pb.len();

        // Track positions that are '*', avoids pb[j-2] bounds checks
        let mut star_of: Vec<bool> = vec![false; n];
        for j in 1..n {
            // '*' cannot be first; safe subtract
            star_of[j] = pb[j] == b'*';
        }

        // One-row DP buffer, +1 for empty prefix
        let mut dp: Vec<bool> = vec![false; n + 1];
        dp[0] = true;

        // Pre-handle patterns like a*, a*b*, etc. for empty s
        for j in 1..=n {
            if star_of[j - 1] {
                dp[j] = dp[j - 2];
            }
        }

        // Iterate through characters of s
        for i in 1..=m {
            // prev holds dp[i-1][j-1]
            let mut prev: bool = dp[0];

            // Non-empty s cannot match empty pattern
            dp[0] = false;

            // Track if any dp[j] becomes true to enable pruning
            let mut any_true: bool = false;

            // Traverse pattern
            for j in 1..=n {
                let temp = dp[j];

                if !star_of[j - 1] {
                    // Non-star case
                    dp[j] = prev && (pb[j - 1] == b'.' || pb[j - 1] == sb[i - 1]);
                } else {
                    // '*' case
                    let zero: bool = dp[j - 2];
                    let one: bool = dp[j] && (pb[j - 2] == b'.' || pb[j - 2] == sb[i - 1]);
                    dp[j] = zero || one;
                }

                if dp[j] {
                    any_true = true;
                }

                // shift diagonal
                prev = temp;
            }

            // Early exit if no pattern prefix matches current s-prefix
            if !any_true {
                return false;
            }
        }

        // Final cell tells if full pattern matches full string
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_010_1_empty_empty() {
        let s = "".to_string();
        let p = "".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn test_010_2_empty_star() {
        let s = "".to_string();
        let p = "a*".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn test_010_3_empty_dot_star() {
        let s = "".to_string();
        let p = ".*".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn test_010_4_nonempty_empty() {
        let s = "a".to_string();
        let p = "".to_string();
        assert!(!Solution::is_match(s, p));
    }

    #[test]
    fn test_010_5_exact_match() {
        let s = "abc".to_string();
        let p = "abc".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn test_010_6_dot_match() {
        let s = "abc".to_string();
        let p = "a.c".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn test_010_7_star_zero() {
        let s = "ab".to_string();
        let p = "ac*b".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn test_010_8_star_multiple() {
        let s = "aaa".to_string();
        let p = "a*".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn test_010_9_no_match() {
        let s = "ab".to_string();
        let p = ".*c".to_string();
        assert!(!Solution::is_match(s, p));
    }

    #[test]
    fn test_010_10_cstar_a_star_b() {
        let s = "aab".to_string();
        let p = "c*a*b".to_string();
        assert!(Solution::is_match(s, p));
    }
}
