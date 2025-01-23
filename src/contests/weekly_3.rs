use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut i, mut j) = (0, 0);
        let (sb, tb) = (s.as_bytes(), t.as_bytes());

        while i < sb.len() && j < tb.len() {
            if sb[i] == tb[j] {
                i += 1;
            }

            j += 1;
        }

        i == sb.len()
    }

    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut n_bytes: i32 = 0;

        let mask1: i32 = 1 << 7;
        let mask2: i32 = 1 << 6;

        for num in data {
            if n_bytes == 0 {
                let mut mask: i32 = 1 << 7;

                while mask & num != 0 {
                    n_bytes += 1;
                    mask = mask >> 1;
                }

                if n_bytes == 0 {
                    continue;
                }

                if n_bytes == 1 || n_bytes > 4 {
                    return false;
                }
            } else {
                if !((num & mask1) != 0 && (mask2 & num) == 0) {
                    return false;
                }
            }

            n_bytes -= 1;
        }

        n_bytes == 0
    }

    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(usize, String)> = Vec::new();
        let (mut n, mut res) = (0, String::new());

        for c in s.chars() {
            match c {
                '[' => {
                    stack.push((n, res.clone()));
                    n = 0;
                    res.clear();
                }

                ']' => {
                    if let Some(last) = stack.pop() {
                        res = last.1 + res.repeat(last.0).as_str();
                    }
                }

                '0'..='9' => n = n * 10 + c.to_digit(10).unwrap() as usize,

                c => res.push(c),
            }
        }

        res
    }

    pub fn longest_substring(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let max_unique: usize = Self::get_unique_chars(&chars);
        let mut result: i32 = 0;

        for unique in 1..=max_unique {
            let (mut start, mut end) = (0, 0);
            let mut curr_unique: usize = 0;
            let mut count: Vec<i32> = vec![0; 26];
            let mut count_k: usize = 0;

            while end < chars.len() {
                if curr_unique <= unique {
                    if count[chars[end] as usize - 'a' as usize] == 0 {
                        curr_unique += 1;
                    }

                    count[chars[end] as usize - 'a' as usize] += 1;

                    if count[chars[end] as usize - 'a' as usize] == k {
                        count_k += 1;
                    }

                    end += 1;
                } else {
                    if count[chars[start] as usize - 'a' as usize] == k {
                        count_k -= 1;
                    }

                    count[chars[start] as usize - 'a' as usize] -= 1;

                    if count[chars[start] as usize - 'a' as usize] == 0 {
                        curr_unique -= 1;
                    }

                    start += 1;
                }

                if curr_unique == unique && curr_unique == count_k {
                    result = result.max((end - start) as i32);
                }
            }
        }

        result
    }

    fn get_unique_chars(chars: &Vec<char>) -> usize {
        chars
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    }
}

pub struct SolutionAlt;

impl SolutionAlt {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let str: &[u8] = s.as_bytes();
        let mut count_map: [i32; 26] = [0; 26];
        let max_unique: usize = SolutionAlt::get_max_unique_letters(&s);
        let mut result: usize = 0;

        for curr_unique in 1..=max_unique {
            count_map.fill(0);
            let mut window_start: usize = 0;
            let mut window_end: usize = 0;
            let mut unique: usize = 0;
            let mut count_at_least_k: usize = 0;

            while window_end < str.len() {
                if unique <= curr_unique {
                    let idx: usize = (str[window_end] - b'a') as usize;

                    if count_map[idx] == 0 {
                        unique += 1;
                    }

                    count_map[idx] += 1;

                    if count_map[idx] == k {
                        count_at_least_k += 1;
                    }

                    window_end += 1;
                } else {
                    let idx: usize = (str[window_start] - b'a') as usize;

                    if count_map[idx] == k {
                        count_at_least_k -= 1;
                    }

                    count_map[idx] -= 1;

                    if count_map[idx] == 0 {
                        unique -= 1;
                    }

                    window_start += 1;
                }

                if unique == curr_unique && unique == count_at_least_k {
                    result = result.max(window_end - window_start);
                }
            }
        }

        result as i32
    }

    fn get_max_unique_letters(s: &String) -> usize {
        s.chars().collect::<HashSet<char>>().len()
    }
}
