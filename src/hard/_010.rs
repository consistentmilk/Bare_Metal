/**
Intuition:
- Collapse the 2D DP table into one 1D array of length (n+1).
- Use a `prev` variable to hold the "diagonal" (dp[i-1][j-1]) value.
- Handle '*' via two cases: zero occurrences (dp[j-2]) or
  one-or-more occurrences (prev dp[j] & char match).

Algorithm:
1. Convert `s` and `p` to byte arrays.
2. Init `dp[0]=true`; pre-process pattern '*' for empty `s`.
3. For each `i` in 1..=m:
   a. Set `prev = dp[0]`, then `dp[0]=false`.
   b. For each `j` in 1..=n, use `temp = dp[j]` to
      update `dp[j]` by the recurrence above, then
      set `prev = temp`.
4. Return `dp[n]`.

Time Complexity: O(m * n)
Space Complexity: O(n)
*/
pub struct Solution;

impl Solution {
    // High-performance 1D-DP matcher
    pub fn is_match(s: String, p: String) -> bool {
        // convert inputs to byte slices
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();

        // lengths of string and pattern
        let m = s_bytes.len();
        let n = p_bytes.len();

        // one-dimensional DP array
        let mut dp = vec![false; n + 1];

        // empty string matches empty pattern
        dp[0] = true;

        // handle patterns like a*, a*b*, etc., matching empty s
        for j in 1..=n {
            // if current pattern char is '*'
            if p_bytes[j - 1] == b'*' {
                dp[j] = dp[j - 2];
            }
        }

        // iterate over each character of s
        for i in 1..=m {
            // prev holds dp[i-1][j-1] initially dp[i-1][0]
            let mut prev = dp[0];
            // non-empty s cannot match empty pattern
            dp[0] = false;

            for j in 1..=n {
                // store old dp[j] = dp[i-1][j]
                let temp = dp[j];

                if p_bytes[j - 1] != b'*' {
                    // single char match or '.'
                    dp[j] = prev && (p_bytes[j - 1] == b'.' || p_bytes[j - 1] == s_bytes[i - 1]);
                } else {
                    // '*' zero occurrences of p[j-2]
                    let zero = dp[j - 2];
                    // '*' one-or-more occurrences
                    let one = temp && (p_bytes[j - 2] == b'.' || p_bytes[j - 2] == s_bytes[i - 1]);
                    dp[j] = zero || one;
                }

                // update prev for next column
                prev = temp;
            }
        }

        // final cell holds the answer
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
