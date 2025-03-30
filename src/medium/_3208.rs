pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n: usize = colors.len();
        let k: usize = k as usize;

        let upper_bound: usize = n + k - 1;
        let mut group_count: i32 = 0;
        let mut curr_alt_count: usize = 1;

        let mut last_color: i32 = colors[0];

        for i in 1..upper_bound {
            let idx: usize = if i >= n { i - n } else { i };

            if colors[idx] == last_color {
                curr_alt_count = 1;
                last_color = colors[idx];
                continue;
            }

            curr_alt_count += 1;

            if curr_alt_count >= k {
                group_count += 1;
            }

            last_color = colors[idx];
        }

        group_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3208_1() {
        let colors: Vec<i32> = vec![0, 1, 0, 1, 0];
        let k: i32 = 3;
        let expected: i32 = 3;
        let res: i32 = Solution::number_of_alternating_groups(colors, k);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_3208_2() {
        let colors: Vec<i32> = vec![0, 1, 0, 0, 1, 0, 1];
        let k: i32 = 6;
        let expected: i32 = 2;
        let res: i32 = Solution::number_of_alternating_groups(colors, k);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_3208_3() {
        let colors: Vec<i32> = vec![1, 1, 0, 1];
        let k: i32 = 4;
        let expected: i32 = 0;
        let res: i32 = Solution::number_of_alternating_groups(colors, k);

        assert_eq!(expected, res);
    }
}
