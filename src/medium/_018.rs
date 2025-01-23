pub struct Solution;

impl Solution {
    pub fn four_sum(mut xs: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if xs.len() < 4 {
            return vec![];
        }

        xs.sort_unstable();

        let len_xs: usize = xs.len();
        let mut ys: Vec<Vec<i32>> = vec![];

        fn f(a: i32, b: i32, c: i32, d: i32) -> Option<i32> {
            a.checked_add(b)?.checked_add(c)?.checked_add(d)
        }

        for i in (3..len_xs).rev() {
            if i != len_xs - 1 && xs[i] == xs[i + 1] {
                continue;
            }

            if let Some(x) = f(xs[i], xs[i - 1], xs[i - 2], xs[i - 3]) {
                if x < target {
                    break;
                }
            }

            if let Some(x) = f(xs[i], xs[0], xs[1], xs[2]) {
                if x > target {
                    continue;
                }
            }

            for j in (2..i).rev() {
                if j != i - 1 && xs[j] == xs[j + 1] {
                    continue;
                }

                if let Some(x) = f(xs[i], xs[j], xs[j - 1], xs[j - 2]) {
                    if x < target {
                        break;
                    }
                }

                if let Some(x) = f(xs[i], xs[j], xs[0], xs[1]) {
                    if x > target {
                        continue;
                    }
                }

                let mut k: usize = j - 1;
                let mut l: usize = 0;

                while l < k {
                    if let Some(x) = f(xs[i], xs[j], xs[k], xs[l]) {
                        if x == target {
                            ys.push(vec![xs[l], xs[k], xs[j], xs[i]]);

                            while l < k - 1 && xs[l] == xs[l + 1] {
                                l += 1;
                            }

                            l += 1;
                        } else if x > target {
                            k -= 1;
                        } else {
                            l += 1;
                        }
                    } else {
                        k -= 1;
                    }
                }
            }
        }
        ys
    }
}

pub struct AltSolution;

// Fails only one test case
impl AltSolution {
    pub fn four_sum(mut xs: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        fn k_sum(nums: Vec<i32>, target: i32, k: i32) -> Vec<Vec<i32>> {
            let mut res: Vec<Vec<i32>> = Vec::new();

            if nums.is_empty() {
                return res;
            }

            let avg_val: i32 = target / k;

            if avg_val < nums[0] || nums[nums.len() - 1] < avg_val {
                return res;
            }

            if k == 2 {
                return two_sum(nums, target);
            }

            for i in 0..nums.len() {
                if i == 0 || nums[i - 1] != nums[i] {
                    let subsets: Vec<Vec<i32>> =
                        k_sum(nums[i + 1..].to_vec(), target - nums[i], k - 1);

                    for subset in subsets {
                        let mut temp: Vec<i32> = vec![];
                        temp.push(nums[i]);
                        temp.extend(subset);
                        res.push(temp);
                    }
                }
            }

            res
        }

        fn two_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            let mut res: Vec<Vec<i32>> = Vec::new();
            let mut s: HashSet<i32> = HashSet::new();

            for i in 0..nums.len() {
                if res.len() == 0 || res[res.len() - 1][1] != nums[i] {
                    if let Some(comp) = target.checked_sub(nums[i]) {
                        if s.contains(&comp) {
                            res.push(vec![target - nums[i], nums[i]]);
                        }
                    } else {
                        return vec![];
                    }
                }

                s.insert(nums[i]);
            }

            res
        }

        xs.sort_unstable();

        k_sum(xs, target, 4)
    }
}
