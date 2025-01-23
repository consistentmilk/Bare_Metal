pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack
            .find(&needle)
            .map(|val: usize| val as i32)
            .unwrap_or(-1)
    }
}
