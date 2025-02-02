pub struct Solution;

///
/// PAYPALISHIRING
///
/// P           I           N
/// A       L   S       I   G
/// Y   A       H   R
/// P           I   
///
/// PAYPALISHIRING -> PINALSIGYAHRPI
///
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows: usize = num_rows as usize;

        if num_rows == 1 || num_rows >= s.len() {
            return s;
        }

        let mut current_row: usize = 0;
        let mut dp: Vec<String> = vec!["".into(); num_rows];
        let mut direction: i32 = -1;

        for ch in s.chars() {
            dp[current_row].push(ch);

            if current_row == 0 || current_row == num_rows - 1 {
                direction *= -1;
            }

            current_row = (current_row as i32 + direction) as usize;
        }

        dp.iter()
            .flat_map(|s: &String| s.as_str().chars())
            .collect()
    }
}

pub struct SolutionOpt;

impl SolutionOpt {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let n: usize = s.len();
        let sbytes: &[u8] = s.as_bytes();
        let num_rows: usize = num_rows as usize;
        let max_jump: usize = 2 * (num_rows - 1);
        let mut res: Vec<u8> = Vec::with_capacity(n);

        for mut i in 0..num_rows {
            let mut jump_even = max_jump - 2 * i;
            let mut jump_odd = 2 * i;

            if jump_even == 0 {
                jump_even = max_jump;
            }

            if jump_odd == 0 {
                jump_odd = max_jump;
            }

            let mut is_even: bool = true;

            loop {
                if i >= n {
                    break;
                }

                res.push(sbytes[i]);
                i += if is_even { jump_even } else { jump_odd };
                is_even = !is_even;
            }
        }

        unsafe { String::from_utf8_unchecked(res) }
    }
}

pub struct SolutionAlt;

impl SolutionAlt {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let s = s.into_bytes();
        let num_rows = num_rows as usize;
        let period = num_rows * 2 - 2;
        let mut answer = Vec::with_capacity(s.len());

        for y in (0..s.len()).step_by(period) {
            answer.push(s[y]);
        }

        for x in 1..(num_rows - 1) {
            for y0 in (0..s.len()).step_by(period) {
                if y0 + x < s.len() {
                    answer.push(s[y0 + x]);
                } else {
                    break;
                }

                if y0 + period - x < s.len() {
                    answer.push(s[y0 + period - x]);
                } else {
                    break;
                }
            }
        }

        for y in ((num_rows - 1)..s.len()).step_by(period) {
            answer.push(s[y]);
        }

        unsafe { String::from_utf8_unchecked(answer) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6_1() {
        let test_str: String = "PAYPALISHIRING".into();
        let test_num_rows: i32 = 3;
        let expected: String = "PAHNAPLSIIGYIR".into();

        assert_eq!(SolutionOpt::convert(test_str, test_num_rows), expected);
    }

    #[test]
    fn test_6_2() {
        let test_str: String = "PAYPALISHIRING".into();
        let test_num_rows: i32 = 4;
        let expected: String = "PINALSIGYAHRPI".into();

        assert_eq!(SolutionOpt::convert(test_str, test_num_rows), expected);
    }
}
