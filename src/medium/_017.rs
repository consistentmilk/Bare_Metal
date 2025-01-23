pub struct Solution;

const MAPPING: [std::ops::RangeInclusive<u8>; 8] = [
    (b'a'..=b'c'),
    (b'd'..=b'f'),
    (b'g'..=b'i'),
    (b'j'..=b'l'),
    (b'm'..=b'o'),
    (b'p'..=b's'),
    (b't'..=b'v'),
    (b'w'..=b'z'),
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        digits
            .as_bytes()
            .iter()
            .fold(vec![String::new()], |acc: Vec<String>, x: &u8| {
                acc.iter()
                    .flat_map(|s: &String| {
                        std::iter::repeat(s)
                            .zip(MAPPING[(*x - b'2') as usize].clone())
                            .map(|(s, b)| s.chars().chain(std::iter::once(b as char)).collect())
                            .collect::<Vec<String>>()
                    })
                    .collect()
            })
    }
}
