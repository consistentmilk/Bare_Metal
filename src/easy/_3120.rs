pub struct Solution {}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        use std::collections::HashSet;

        let mut set_lower: HashSet<char> = HashSet::new();
        let mut set_upper: HashSet<char> = HashSet::new();
        let mut set_counted: HashSet<char> = HashSet::new();
        let mut total: i32 = 0;

        for ch in word.chars() {
            if ch.is_ascii_lowercase() && !set_lower.contains(&ch) {
                set_lower.insert(ch);

                if set_upper.contains(&ch.to_ascii_uppercase()) && !set_counted.contains(&ch) {
                    total += 1;
                    set_counted.insert(ch);
                }
            }

            if ch.is_ascii_uppercase() && !set_upper.contains(&ch) {
                set_upper.insert(ch);

                let ch_alt: char = ch.to_ascii_lowercase();

                if set_lower.contains(&ch_alt) && !set_counted.contains(&ch_alt) {
                    total += 1;
                    set_counted.insert(ch_alt);
                }
            }
        }

        total
    }

    pub fn number_of_special_chars_alt(word: String) -> i32 {
        word.chars()
            .fold([0; 26], |mut map: [i32; 26], c: char| {
                let i: usize = c.to_ascii_lowercase() as usize - 97; // 'a'

                if c.is_uppercase() {
                    map[i] |= 0b10;
                } else {
                    map[i] |= 0b01;
                }

                map
            })
            .iter()
            .filter(|x: &&i32| x.eq(&&0b11))
            .count() as i32
    }
}
