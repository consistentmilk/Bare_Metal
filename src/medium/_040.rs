pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current: Vec<i32> = vec![];

        candidates.sort_unstable();

        Solution::backtracking(&candidates, target, &mut result, &mut current);

        result
    }

    fn backtracking(
        candidates: &[i32],
        target: i32,
        result: &mut Vec<Vec<i32>>,
        current: &mut Vec<i32>,
    ) {
        if 0 == target {
            result.push(current.clone());
        } else if target < 0 {
            return;
        }

        let mut prev: i32 = -1;
        for (i, &element) in candidates.iter().enumerate() {
            if prev == element {
                continue;
            }

            current.push(element);

            Solution::backtracking(&candidates[i + 1..], target - element, result, current);

            current.pop();

            prev = element;
        }
    }
}
