pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        use std::str::SplitAsciiWhitespace;
        let s: SplitAsciiWhitespace<'_> = s.trim_end().split_ascii_whitespace();

        if let Some(word) = s.rev().next() {
            if let Some(parsed) = word.strip_suffix('.') {
                return parsed.len() as i32;
            } else {
                return word.len() as i32;
            }
        } else {
            return 0;
        }
    }
}
