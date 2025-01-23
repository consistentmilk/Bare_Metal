pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut ret: i64 = 0;
        let nums1: Vec<i32> = nums1
            .into_iter()
            .filter_map(|n: i32| if n % k == 0 { Some(n / k) } else { None })
            .collect();
        let max: i32 = *nums1.iter().max().unwrap_or(&0);

        let mut nums1_counts: HashMap<i32, i64> = HashMap::new();

        for num in nums1 {
            *nums1_counts.entry(num).or_insert(0) += 1;
        }

        for (num, count) in
            nums2
                .into_iter()
                .fold(HashMap::new(), |mut map: HashMap<i32, i64>, num: i32| {
                    *map.entry(num).or_insert(0) += 1;
                    map
                })
        {
            let mut key: i32 = num;

            while key <= max {
                if let Some(&c2) = nums1_counts.get(&key) {
                    ret += c2 * count;
                }

                key += num;
            }
        }

        ret
    }
}
