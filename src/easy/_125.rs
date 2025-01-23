pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_cleaned: String = s
            .chars()
            .filter(|ch: &char| ch.is_ascii_alphanumeric())
            .map(|ch: char| ch.to_ascii_lowercase())
            .collect();

        s_cleaned.chars().rev().collect::<String>() == s_cleaned
    }
}
