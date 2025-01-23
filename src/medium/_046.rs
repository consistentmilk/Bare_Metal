pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        Self::backtrack(nums, vec![], &mut result);

        result
    }

    fn backtrack(nums: Vec<i32>, path: Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if nums.is_empty() {
            result.push(path);

            return;
        }

        for i in 0..nums.len() {
            let mut new_nums: Vec<i32> = nums.clone();

            new_nums.remove(i);

            let mut new_path: Vec<i32> = path.clone();

            new_path.push(nums[i]);

            Self::backtrack(new_nums, new_path, result);
        }
    }

    pub fn permute_alt(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len: usize = nums.len();

        let mut permutations: Vec<Vec<i32>> = vec![nums.clone()];

        let mut p: Vec<usize> = (0..=len).collect();
        let mut i: usize = 1;
        let mut j: usize;

        while i < len {
            p[i] -= 1;
            j = (i % 2) * p[i];

            let tmp: i32 = nums[i];
            nums[i] = nums[j];
            nums[j] = tmp;

            permutations.push(nums.clone());

            i = 1;

            while p[i] == 0 {
                p[i] = i;
                i += 1;
            }
        }
        permutations
    }
}
