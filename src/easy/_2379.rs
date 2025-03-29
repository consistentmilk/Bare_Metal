pub struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let block_bytes: &[u8] = blocks.as_bytes();
        let white_value: i32 = b'W' as i32;
        let black_value: i32 = b'B' as i32;
        let diff: i32 = white_value - black_value;
        let mut glob_min: i32 = i32::MAX;

        for block in block_bytes.windows(k as usize) {
            let curr_min: i32 = block.iter().map(|x| *x as i32).sum::<i32>() - black_value * k;

            if curr_min == 0 {
                return curr_min;
            }

            glob_min = std::cmp::min(glob_min, curr_min / diff);
        }

        glob_min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2379_1() {
        let s: String = "WBBWWBBWBW".into();
        let k: i32 = 7;
        let expected: i32 = 3;
        let res: i32 = Solution::minimum_recolors(s, k);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_2379_2() {
        let s: String = "WBWBBBW".into();
        let k: i32 = 2;
        let expected: i32 = 0;
        let res: i32 = Solution::minimum_recolors(s, k);

        assert_eq!(expected, res);
    }
}
