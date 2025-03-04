pub struct Solution;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m: usize = nums1.len();
        let n: usize = nums2.len();

        let mut res: Vec<Vec<i32>> = Vec::with_capacity(m + n);
        let mut p1: usize = 0;
        let mut p2: usize = 0;

        while p1 < m && p2 < n {
            let [idx_1, val_1] = nums1[p1][..] else {
                panic!("Safe due to restrictions.")
            };

            let [idx_2, val_2] = nums2[p2][..] else {
                panic!("Safe due to restrictions.")
            };

            if idx_1 == idx_2 {
                res.push(vec![idx_1, val_1 + val_2]);
                p1 += 1;
                p2 += 1;
            } else if idx_1 < idx_2 {
                res.push(nums1[p1].clone());
                p1 += 1;
            } else {
                res.push(nums2[p2].clone());
                p2 += 1;
            }
        }

        if p1 < m {
            res.extend_from_slice(&nums1[p1..]);
        } else if p2 < n {
            res.extend_from_slice(&nums2[p2..]);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2570_1() {
        let nums1: Vec<Vec<i32>> = [[1, 2], [2, 3], [4, 5]]
            .into_iter()
            .map(|x: [i32; 2]| x.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let nums2: Vec<Vec<i32>> = [[1, 4], [3, 2], [4, 1]]
            .into_iter()
            .map(|x: [i32; 2]| x.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let expected: Vec<Vec<i32>> = [[1, 6], [2, 3], [3, 2], [4, 6]]
            .into_iter()
            .map(|x: [i32; 2]| x.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let res: Vec<Vec<i32>> = Solution::merge_arrays(nums1, nums2);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_2570_2() {
        let nums1: Vec<Vec<i32>> = [[2, 4], [3, 6], [5, 5]]
            .into_iter()
            .map(|x: [i32; 2]| x.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let nums2: Vec<Vec<i32>> = [[1, 3], [4, 3]]
            .into_iter()
            .map(|x: [i32; 2]| x.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let expected: Vec<Vec<i32>> = [[1, 3], [2, 4], [3, 6], [4, 3], [5, 5]]
            .into_iter()
            .map(|x: [i32; 2]| x.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let res: Vec<Vec<i32>> = Solution::merge_arrays(nums1, nums2);

        assert_eq!(expected, res);
    }
}
