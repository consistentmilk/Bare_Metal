pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s3.len() != s1.len() + s2.len() {
            return false;
        }

        let s1: &[u8] = s1.as_bytes();
        let s2: &[u8] = s2.as_bytes();
        let s3: &[u8] = s3.as_bytes();

        let mut table: Vec<Vec<bool>> = vec![vec![false; s2.len() + 1]; s1.len() + 1];

        for i in 0..s1.len() + 1 {
            for j in 0..s2.len() + 1 {
                if i == 0 && j == 0 {
                    table[i][j] = true;
                } else if i == 0 {
                    table[i][j] = table[i][j - 1] && s2[j - 1] == s3[i + j - 1];
                } else if j == 0 {
                    table[i][j] = table[i - 1][j] && s1[i - 1] == s3[i + j - 1];
                } else {
                    table[i][j] = (table[i - 1][j] && s1[i - 1] == s3[i + j - 1])
                        || (table[i][j - 1] && s2[j - 1] == s3[i + j - 1]);
                }
            }
        }

        table[s1.len()][s2.len()]
    }

    pub fn is_interleave_alt(s1: String, s2: String, s3: String) -> bool {
        // Cache string lengths for convenience
        let (m, n) = (s1.len(), s2.len());
        // Early exit if impossible
        if s3.len() != m + n {
            return false;
        }

        // Input is ASCII => chars are bytes
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());

        // Build first row of DP table with iterators - true as long as the prefixes of s2 and s3 are equal
        let mut dp_prev: Vec<bool> = std::iter::once(true)
            .chain(s2.iter().zip(s3.iter()).scan(true, |ok, (b2, b3)| {
                *ok &= *b2 == *b3;
                Some(*ok)
            }))
            .collect::<Vec<_>>();

        // Initialize row to compute
        let mut dp_curr: Vec<bool> = vec![false; n + 1];
        for i in 0..m {
            // First entry of every row is true if prefixes of s1 and s3 are the same
            dp_curr[0] = dp_prev[0] && s1[i] == s3[i];
            // Check if we can advance by adding a character from s1 or s2 to an
            // already valid path.
            for j in 1..=n {
                dp_curr[j] = (dp_prev[j] && s1[i] == s3[i + j])
                    || (dp_curr[j - 1] && s2[j - 1] == s3[i + j]);
            }

            // Done with dp_prev - reuse it to avoid additional allocations
            std::mem::swap(&mut dp_curr, &mut dp_prev);
        }
        // Last row of DP table is now in dp_prev due to last swap
        dp_prev[n]
    }
}
