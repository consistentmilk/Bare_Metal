// ideas:
// A valid ip address would have 4 parts separated by dots
// we iterate through `s` to insert 3 dots and separate the string into 4 segments
// for each segment, we check if it is valid
// if all 4 segments are valid, we combine those 4 segments with dots and push to the answer

pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ans: Vec<String> = vec![];
        let n: usize = s.len();

        for i in 1..4 {
            for j in i + 1..i + 4 {
                for k in j + 1..j + 4 {
                    if k < n {
                        let seg1: &str = &s[..i];
                        let seg2: &str = &s[i..j];
                        let seg3: &str = &s[j..k];
                        let seg4: &str = &s[k..];

                        if Solution::valid_seg(seg1)
                            && Solution::valid_seg(seg2)
                            && Solution::valid_seg(seg3)
                            && Solution::valid_seg(seg4)
                        {
                            ans.push(format!("{}.{}.{}.{}", seg1, seg2, seg3, seg4));
                        }
                    }
                }
            }
        }
        ans
    }

    fn valid_seg(seg: &str) -> bool {
        seg.len() > 0
            && seg.len() <= 3
            && !(seg.starts_with('0') && seg.len() > 1)
            && seg.parse::<u32>().unwrap() <= 255
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_093_one() {
        let s: String = "25525511135".to_string();
        let expected: Vec<String> = ["255.255.11.135", "255.255.111.35"]
            .into_iter()
            .map(|s: &str| s.to_string())
            .collect();

        assert_eq!(Solution::restore_ip_addresses(s), expected);
    }

    #[test]
    fn test_093_two() {
        let s: String = "101023".to_string();
        let expected: Vec<String> = [
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3",
        ]
        .into_iter()
        .map(|s: &str| s.to_string())
        .collect();

        assert_eq!(Solution::restore_ip_addresses(s), expected);
    }
}
