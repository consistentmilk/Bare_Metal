pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combinations: Vec<Vec<i32>> = vec![];
        let mut curr: Vec<i32> = vec![];

        Self::helper(&candidates, &mut curr, 0, target, &mut combinations);
        combinations
    }

    fn helper(
        candidates: &Vec<i32>,
        curr: &mut Vec<i32>,
        index: usize,
        target: i32,
        combinations: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            combinations.push(curr.clone());
        }

        if target > 0 {
            for i in index..candidates.len() {
                curr.push(candidates[i]);
                Self::helper(candidates, curr, i, target - candidates[i], combinations);
                curr.pop();
            }
        }
    }
}
