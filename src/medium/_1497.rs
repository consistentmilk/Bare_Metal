// Check If Array Pairs Are Divisible by k
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_arrange_self(arr: Vec<i32>, k: i32) -> bool {
        // [1, 2, 3, 4, 5, 10, 6, 7, 8, 9]
        // [1, 2, 3, 4, 0,  0, 1, 2, 3, 4]
        let mod_arr: Vec<i32> = arr.iter().map(|x| ((x % k) + k) % k).collect();
        let mut freq_map: HashMap<i32, i32> = HashMap::new();

        for i in 0..k {
            let mut count: i32 = 0;

            for val in &mod_arr {
                if *val == i {
                    count += 1;
                }
            }

            freq_map.insert(i, count);
        }

        for i in 0..(k / 2) {
            if i == 0 || i == k - i - 1 {
                let val = freq_map[&i];

                if val == 0 {
                    continue;
                }

                if val % 2 != 0 {
                    return false;
                }
            }

            let lp = freq_map[&i];
            let rp = freq_map[&(k - i - 1)];

            if lp != rp {
                return false;
            }
        }

        true
    }

    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut rem = vec![0; 100000];
        arr.iter().for_each(|&x| rem[x.rem_euclid(k) as usize] += 1);

        for x in 1..=k as usize >> 1 {
            if rem[x] != rem[k as usize - x] {
                return false;
            }
        }

        rem[0] % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1497_1() {
        let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9];
        let k: i32 = 5;

        assert!(Solution::can_arrange(arr, k));
    }
}
