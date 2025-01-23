pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a: &Vec<i32>, b: &Vec<i32>| a[0].cmp(&b[0]));

        let mut merged: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        let mut curr_interval: Vec<i32> = intervals[0].clone();

        for interval in intervals.into_iter().skip(1) {
            if curr_interval[1] >= interval[0] {
                curr_interval[1] = curr_interval[1].max(interval[1]);
            } else {
                merged.push(curr_interval);
                curr_interval = interval;
            }
        }

        merged.push(curr_interval);
        merged
    }
}
