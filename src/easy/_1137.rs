use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::with_capacity((n + 1) as usize);

        fn recurse(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            match n {
                0 => return 0,
                1 | 2 => return 1,
                _ => {
                    if let Some(memoized) = memo.get(&n) {
                        return *memoized;
                    }

                    let res: i32 =
                        recurse(n - 1, memo) + recurse(n - 2, memo) + recurse(n - 3, memo);
                    memo.insert(n, res);

                    res
                }
            }
        }

        recurse(n, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1137_1() {
        let n: i32 = 4;
        assert_eq!(4, Solution::tribonacci(n));
    }

    #[test]
    fn test_1137_2() {
        let n: i32 = 25;
        assert_eq!(1389537, Solution::tribonacci(n));
    }
}
