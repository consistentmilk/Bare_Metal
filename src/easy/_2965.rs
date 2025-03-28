pub struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n: usize = grid.len();

        let mut res: Vec<i32> = vec![0, 0];
        let mut num_tbl: Vec<i32> = vec![0; n.pow(2) + 1];

        for row in grid {
            for val in row {
                if num_tbl[val as usize] == 0 {
                    num_tbl[val as usize] += 1;
                } else {
                    res[0] = val;
                }
            }
        }

        res[1] = num_tbl[1..]
            .iter()
            .position(|x: &i32| *x == 0)
            .expect("Always exists due to restrictions") as i32;
        res[1] += 1;

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2965_1() {
        let input: Vec<Vec<i32>> = [[1, 3], [2, 2]]
            .iter()
            .map(|x: &[i32; 2]| x.to_vec())
            .collect();
        let expected: Vec<i32> = vec![2, 4];
        let res: Vec<i32> = Solution::find_missing_and_repeated_values(input);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_2965_2() {
        let input: Vec<Vec<i32>> = [[9, 1, 7], [8, 9, 2], [3, 4, 6]]
            .iter()
            .map(|x: &[i32; 3]| x.to_vec())
            .collect();
        let expected: Vec<i32> = vec![9, 5];
        let res: Vec<i32> = Solution::find_missing_and_repeated_values(input);

        assert_eq!(expected, res);
    }
}
