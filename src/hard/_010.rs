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
1. Convert s and p to byte slices for O(1) char access.
2. Build star_of[] marking '*' in pattern.
3. Initialize dp[0..=n] with dp[0] = true.
4. Pre-fill dp for empty s matching patterns (x*, x*y*, ...).
5. For i in 1..=m:
    a. Save prev = dp[0], set dp[0] = false, any_true = false.
    b. For j in 1..=n:
    - If p[j-1] != '*':
        dp[j] = prev
            && (pb[j-1] == b'.'
                || pb[j-1] == sb[i-1])
    - Else:
        dp[j] = dp[j-2]
            || (dp[j]
                && (pb[j-2] == b'.'
                    || pb[j-2] == sb[i-1]))
    - Update any_true, prev.
    c. If !any_true, return false.
6. Return dp[n] as final result.

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
                let temp: bool = dp[j];

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

/*
Intuition:
  Build a 2D table dp where dp[i][j] is true if s[i:]
  matches p[j:], filling from end to start.
Algorithm:
  1. Convert s,p to byte slices sb,pb.
  2. Let m=sb.len(), n=pb.len().
  3. Create dp table of size (m+1)x(n+1), initialized false.
  4. Set dp[m][n]=true for empty vs empty.
  5. Pre-fill dp[m][j] for patterns where p[j+1]=='*':
       dp[m][j] = dp[m][j+2].
  6. For i from m-1 down to 0:
       for j from n-1 down to 0:
         first_match = (pb[j]==b'.') || (pb[j]==sb[i])
         if j+1<n && pb[j+1]==b'*':
           dp[i][j] = dp[i][j+2]
                    || (first_match && dp[i+1][j])
         else:
           dp[i][j] = first_match && dp[i+1][j+1]
  7. Return dp[0][0].
Time Complexity: O(m * n)
Space Complexity: O(m * n)
*/

pub struct SolutionAlt;

impl SolutionAlt {
    /// Determine if s matches p with '.' and '*'.
    pub fn is_match(s: String, p: String) -> bool {
        // Convert input string to byte slice.
        let sb: &[u8] = s.as_bytes();
        // Convert pattern string to byte slice.
        let pb: &[u8] = p.as_bytes();
        // Length of string.
        let m: usize = sb.len();
        // Length of pattern.
        let n: usize = pb.len();

        // Create DP table (m+1 rows of n+1 columns).
        let mut dp: Vec<Vec<bool>> = vec![vec![false; n + 1]; m + 1];
        // Base case: empty string matches empty pattern.
        dp[m][n] = true;

        // Key Idea:
        // Matching the empty string "" against pattern p[j..]
        // that starts with "x*"
        // is equivalent to skipping "x*" and trying to match p[j+2..].
        // Pre-fill for empty string matching 'x*' patterns.
        for j in (0..n).rev() {
            if j + 1 < n && pb[j + 1] == b'*' {
                dp[m][j] = dp[m][j + 2];
            }
        }

        // Fill table bottom-up.
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                // Check if current characters match or pattern is '.'.
                let first_match: bool = (pb[j] == b'.') || (pb[j] == sb[i]);

                // Handle '*' wildcard.
                if j + 1 < n && pb[j + 1] == b'*' {
                    // Key Idea
                    // If:
                    //   - The rest of the pattern after "x*" matches (→ skip case)
                    // OR
                    //   - The current character matches and the rest of the string matches
                    //     with the same pattern (→ consume case),
                    // Then:
                    //   - s[i:] matches p[j:]

                    // We match one occurrence of 'x' from s[i]
                    // We stay at j in the pattern because '*' allows repeating 'x'
                    dp[i][j] = dp[i][j + 2] || (first_match && dp[i + 1][j]);
                } else {
                    // No '*': must match current char and rest.
                    // We only succeed at position (i, j) if:
                    // 1. s[i] matches p[j]
                    // 2. Th rest of the string matches the rest of the pattern.
                    dp[i][j] = first_match && dp[i + 1][j + 1];
                }
            }
        }

        // Final result for full strings.
        dp[0][0]
    }
}

/*
Intuition:
  Use recursion with HashMap memoization to cache
  only visited (i,j) states and explore necessary branches.

Algorithm:
  1. Convert s,p to byte slices sb,pb.
  2. Create HashMap<(usize,usize),bool> memo.
  3. Define recursive dp(i,j):
     a. Return cached if present.
     b. If j == n, res = i == m.
     c. Compute first_match.
     d. If next is '*', res = dp(i,j+2)
           || (first_match && dp(i+1,j));
        else res = first_match && dp(i+1,j+1).
     e. memo.insert((i,j),res).
  4. Return dp(0,0).

Time Complexity: O(m * n)
Space Complexity: O(m * n) memo + recursion stack
*/

// Import HashMap for memoization
use std::collections::HashMap;

pub struct SolutionRecursive;

impl SolutionRecursive {
    /// Top-down regex match with HashMap memoization
    pub fn is_match(s: String, p: String) -> bool {
        // Convert s to a byte slice for indexing
        let sb: &[u8] = s.as_bytes();

        // Convert p to a byte slice for indexing
        let pb: &[u8] = p.as_bytes();

        // Length of the input string
        let m: usize = sb.len();

        // Length of the pattern string
        let n: usize = pb.len();

        // Memoization map: keys are (i,j) tuples
        let mut memo: HashMap<(usize, usize), bool> = HashMap::new();

        // Recursive helper: does sb[i..] match pb[j..]?
        fn dp(
            i: usize,
            j: usize,
            sb: &[u8],
            pb: &[u8],
            m: &usize,
            n: &usize,
            memo: &mut HashMap<(usize, usize), bool>,
        ) -> bool {
            // Return cached result if it exists
            if let Some(&v) = memo.get(&(i, j)) {
                return v;
            }

            // Will hold the computed result
            let res: bool;
            // If at end of pattern, match only if string done
            if &j == n {
                res = &i == m;
            } else {
                // Check if current chars match or pattern is '.'
                let first_match: bool = (&i < m) && (pb[j] == b'.' || pb[j] == sb[i]);

                // Handle '*' wildcard
                if (&(j + 1) < n) && pb[j + 1] == b'*' {
                    // Case 1: skip "x*"
                    // Case 2: consume one "x" if first_match
                    res = dp(i, j + 2, sb, pb, m, n, memo)
                        || (first_match && dp(i + 1, j, sb, pb, m, n, memo));
                } else {
                    // No '*': must match current and rest
                    res = first_match && dp(i + 1, j + 1, sb, pb, m, n, memo);
                }
            }

            // Cache the result before returning
            memo.insert((i, j), res);

            res
        }

        // Start matching from the beginning of s and p
        dp(0, 0, sb, pb, &m, &n, &mut memo)
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
