use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum_opt(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (idx, val) in nums.iter().enumerate() {
            let complement: i32 = target - val;

            if map.contains_key(&complement) {
                return vec![map[&complement], idx as i32];
            }

            map.insert(*val, idx as i32);
        }

        vec![]
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h_tbl: HashMap<i32, i32> = HashMap::new();

        for (idx, val) in nums.iter().enumerate() {
            let cmp = target - val;

            if let Some(&prev_val) = h_tbl.get(&cmp) {
                return vec![prev_val, idx as i32];
            }

            h_tbl.insert(*val, idx as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_001_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn test_001_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }
}
