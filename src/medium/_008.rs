pub struct Solution {}

#[derive(Debug)]
pub enum State {
    Start,
    SignParsed,
    ParsingDigits,
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut res: i32 = 0;
        let mut state: State = State::Start;
        let mut sign: i32 = 1;
        let char_bytes: &[u8] = s.as_bytes();

        for char_byte in char_bytes.iter() {
            match state {
                State::Start => match char_byte {
                    b' ' => {
                        continue;
                    }

                    b'+' => {
                        state = State::SignParsed;
                    }

                    b'-' => {
                        state = State::SignParsed;
                        sign = -1;
                    }

                    b'0'..=b'9' => {
                        res = res
                            .saturating_mul(10)
                            .saturating_add((*char_byte - b'0') as i32 * sign);
                        state = State::ParsingDigits;
                    }

                    _ => return 0,
                },

                State::SignParsed => match char_byte {
                    b'0'..=b'9' => {
                        res = res
                            .saturating_mul(10)
                            .saturating_add((*char_byte - b'0') as i32 * sign);
                        state = State::ParsingDigits;
                    }

                    _ => return 0,
                },

                State::ParsingDigits => match char_byte {
                    b'0'..=b'9' => {
                        res = res
                            .saturating_mul(10)
                            .saturating_add((*char_byte - b'0') as i32 * sign);
                    }

                    _ => return res,
                },
            }
        }

        res
    }

    pub fn atoi(s: String) -> i32 {
        let s: &str = s.trim_start();

        let (s, sign) = match s.strip_prefix('-') {
            Some(s) => (s, -1),
            None => (s.strip_prefix('+').unwrap_or(s), 1),
        };

        s.chars()
            .map(|ch| ch.to_digit(10))
            .take_while(|res| res.is_some())
            .flatten()
            .fold(0, |acc, digit| {
                acc.saturating_mul(10).saturating_add(sign * digit as i32)
            })
    }

    pub fn _my_atoi(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut value: i32 = 0;
        let mut state: State = State::Start;
        let mut sign: i32 = 1;
        let mut it: std::str::Chars<'_> = s.chars();

        while let Some(curr_ch) = it.next() {
            match state {
                State::Start => match curr_ch {
                    ' ' => {
                        continue;
                    }

                    '-' => {
                        sign = -1;
                        state = State::SignParsed;
                    }

                    '+' => {
                        state = State::SignParsed;
                    }

                    _ => {
                        if let Some(digit) = curr_ch.to_digit(10) {
                            state = State::ParsingDigits;
                            value = value.saturating_mul(10).saturating_add(sign * digit as i32);
                        } else {
                            return 0;
                        }
                    }
                },

                State::SignParsed => {
                    if let Some(digit) = curr_ch.to_digit(10) {
                        state = State::ParsingDigits;
                        value = value.saturating_mul(10).saturating_add(sign * digit as i32);
                    } else {
                        return 0;
                    }
                }

                State::ParsingDigits => {
                    if let Some(digit) = curr_ch.to_digit(10) {
                        value = value.saturating_mul(10).saturating_add(sign * digit as i32);
                    } else {
                        break;
                    }
                }
            }
        }

        value
    }

    pub fn __my_atoi(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut res: i32 = 0;
        let mut sign: i32 = 1;
        let mut state: State = State::Start;

        for ch in s.chars() {
            match state {
                State::Start => match ch {
                    ' ' => {
                        continue;
                    }

                    '+' => {
                        state = State::SignParsed;
                    }

                    '-' => {
                        state = State::SignParsed;
                        sign = -1;
                    }

                    _ => {
                        if let Some(digit) = ch.to_digit(10) {
                            res = res.saturating_mul(10).saturating_add(sign * (digit as i32));
                            state = State::ParsingDigits;
                        } else {
                            return 0;
                        }
                    }
                },

                State::SignParsed => {
                    if let Some(digit) = ch.to_digit(10) {
                        res = res.saturating_mul(10).saturating_add(sign * (digit as i32));
                        state = State::ParsingDigits;
                    } else {
                        return 0;
                    }
                }

                State::ParsingDigits => {
                    if let Some(digit) = ch.to_digit(10) {
                        res = res.saturating_mul(10).saturating_add(sign * (digit as i32));
                    } else {
                        break;
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_008_positive_number() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_008_negative_number() {
        assert_eq!(Solution::my_atoi("-42".to_string()), -42);
    }

    #[test]
    fn test_008_with_spaces() {
        assert_eq!(Solution::my_atoi("   4193".to_string()), 4193);
    }

    #[test]
    fn test_008_with_words() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn test_008_with_overflow() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }

    #[test]
    fn test_008_with_plus_sign() {
        assert_eq!(Solution::my_atoi("+1".to_string()), 1);
    }

    #[test]
    fn test_008_with_plus_and_minus() {
        assert_eq!(Solution::my_atoi("+-12".to_string()), 0);
    }

    #[test]
    fn test_008_with_minus_and_plus() {
        assert_eq!(Solution::my_atoi("-+12".to_string()), 0);
    }

    #[test]
    fn test_008_with_plus_sign_and_words() {
        assert_eq!(Solution::my_atoi("+words and 987".to_string()), 0);
    }

    #[test]
    fn test_008_with_maximum_value() {
        assert_eq!(Solution::my_atoi("2147483648".to_string()), 2147483647);
    }
}
