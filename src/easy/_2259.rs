pub struct Solution;

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut res: String = String::with_capacity(number.len() - 1);
        let mut max: String = "0".into();

        for (idx, ch) in number.char_indices() {
            if ch == digit {
                let mut new_num_str: String = String::with_capacity(number.len() - 1);
                new_num_str.push_str(&number[..idx]);
                new_num_str.push_str(&number[idx + 1..]);

                if new_num_str > max {
                    max = new_num_str.clone();
                    res = new_num_str;
                }
            }
        }

        res
    }

    pub fn remove_digit_alt(mut number: String, digit: char) -> String {
        let mut chars: std::iter::Enumerate<std::str::Chars> = number.chars().enumerate();
        let mut last_seen: usize = 0;
        let (mut i_prev, mut c_prev) = chars.next().unwrap();

        while let Some((i, c)) = chars.next() {
            if c_prev == digit {
                last_seen = i_prev;

                if c.to_digit(10).unwrap() > c_prev.to_digit(10).unwrap() {
                    number.remove(i_prev);
                    return number;
                }
            }

            c_prev = c;
            i_prev = i;
        }

        if number.chars().last().unwrap() == digit {
            number.remove(number.len() - 1);
            return number;
        }

        number.remove(last_seen);
        number
    }
}
