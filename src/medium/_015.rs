pub struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        nums.sort_unstable();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] || nums[i] > 0 {
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                let total = nums[i] + nums[j] + nums[k];

                if total > 0 {
                    k -= 1;
                } else if total < 0 {
                    j += 1;
                } else {
                    let tmp = vec![nums[i], nums[j], nums[k]];
                    res.push(tmp);
                    j += 1;

                    while nums[j] == nums[j - 1] && j < k {
                        j += 1;
                    }
                }
            }
        }

        res
    }
}
