pub struct Solution;

impl Solution {
    pub fn is_match(text: String, pattern: String) -> bool {
        let t_len = text.len();
        let p_len = pattern.len();

        let mut dp: Vec<Vec<bool>> = vec![vec![false; p_len + 1]; t_len + 1];
        dp[t_len][p_len] = true;

        let text = text.as_bytes();
        let pattern = pattern.as_bytes();

        for i in (0..=t_len).rev() {
            for j in (0..p_len).rev() {
                let first_match = i < t_len && (text[i] == pattern[j] || pattern[j] == b'.');

                if j + 1 < p_len && pattern[j + 1] == b'*' {
                    dp[i][j] = dp[i][j + 2] || (first_match && dp[i + 1][j]);
                } else {
                    dp[i][j] = first_match && dp[i + 1][j + 1];
                }
            }
        }

        dp[0][0]
    }

    pub fn is_match_alt_1(s: String, p: String) -> bool {
        let m: usize = s.len();
        let n: usize = p.len();
        let string: Vec<char> = s.chars().collect();
        let pattern: Vec<char> = p.chars().collect();

        let mut matches: Vec<Vec<bool>> = vec![vec![false; n + 1]; m + 1];
        matches[m][n] = true;

        for j in (0..(n - 1)).rev() {
            if pattern[j + 1] == '*' {
                matches[m][j] = matches[m][j + 2];
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let target: char = pattern[j];
                let kleene: bool = pattern.get(j + 1).map_or(false, |&c| c == '*');

                let char_match: bool = target == '.' || target == string[i];

                if !kleene {
                    matches[i][j] = char_match && matches[i + 1][j + 1];
                } else {
                    matches[i][j] = (char_match && matches[i + 1][j]) || matches[i][j + 2];
                }
            }
        }

        matches[0][0]
    }

    pub fn is_match_alt_2(s: String, p: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();
        let (m, n) = (s.len(), p.len());

        let mut dp: Vec<Vec<bool>> = vec![vec![false; n + 1]; 2];
        dp[0][0] = true;

        for j in 1..=n {
            if p_chars[j - 1] == '*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=m {
            let curr: usize = i % 2;
            let prev: usize = 1 - curr;

            dp[curr][0] = false;

            for j in 1..=n {
                if p_chars[j - 1] == s_chars[i - 1] || p_chars[j - 1] == '.' {
                    dp[curr][j] = dp[prev][j - 1];
                } else if p_chars[j - 1] == '*' {
                    dp[curr][j] = dp[curr][j - 2]
                        || (dp[prev][j]
                            && (s_chars[i - 1] == p_chars[j - 2] || p_chars[j - 2] == '.'));
                } else {
                    dp[curr][j] = false;
                }
            }
        }

        dp[m % 2][n]
    }
}
