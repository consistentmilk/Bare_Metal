pub struct Solution;
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
        let mut res: i32 = 0;

        for adj in bytes.windows(2) {
            res += u8::abs_diff(adj[0], adj[1]) as i32;
        }

        res
    }
}
