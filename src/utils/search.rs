use std::cmp::Ordering;

pub struct SearchAlgorithms;

impl SearchAlgorithms {
    /// Finds whether the target element is in the provided Vector,
    /// without mutation
    pub fn binary_search<T>(elems: Vec<T>, target: T, sorted: bool) -> bool
    where
        T: Copy + PartialOrd + Ord,
    {
        if !sorted {
            let mut elems: Vec<T> = elems.clone();
            elems.sort_unstable();

            return Self::binary_search(elems, target, true);
        }

        let mut lp: usize = 0;
        let mut rp: usize = elems.len() - 1;

        while lp <= rp {
            let mid: usize = lp + (rp - lp) / 2;

            match elems[mid].cmp(&target) {
                Ordering::Greater => {
                    rp = mid - 1;
                }

                Ordering::Less => {
                    lp = mid + 1;
                }

                Ordering::Equal => {
                    return true;
                }
            }
        }

        false
    }

    /// Finds the position where the target element is or should be in
    /// the provided Vector.
    pub fn binary_search_position<T>(elems: Vec<T>, target: T, sorted: bool) -> usize
    where
        T: Copy + PartialOrd + Ord,
    {
        if !sorted {
            let mut elems: Vec<T> = elems.clone();
            elems.sort_unstable();

            return Self::binary_search_position(elems, target, true);
        }

        let mut lp: usize = 0;
        let mut rp: usize = elems.len();

        while lp <= rp {
            let mid: usize = lp + (rp - lp) / 2;

            match elems[mid].cmp(&target) {
                Ordering::Greater => {
                    rp = mid - 1;
                }

                Ordering::Less => {
                    lp = mid + 1;
                }

                Ordering::Equal => {
                    return mid;
                }
            }
        }

        lp
    }

    pub fn linear_search<T>(arr: &[T], target: T) -> Option<usize>
    where
        T: Copy + PartialEq,
    {
        for (idx, &val) in arr.iter().enumerate() {
            if val == target {
                return Some(idx);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salgs_one() {
        let test_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        let test_tar: i32 = 3;

        assert!(SearchAlgorithms::binary_search(test_vec, test_tar, true));
    }

    #[test]
    fn test_salgs_two() {
        let test_vec: Vec<i32> = vec![1, 2, 3, 4, 6, 7, 8, 9];
        let test_tar: i32 = 5;

        assert!(!SearchAlgorithms::binary_search(test_vec, test_tar, true));
    }

    #[test]
    fn test_salgs_three() {
        let test_vec: Vec<i32> = vec![3, 1, 2, 7, 4];
        let test_tar: i32 = 3;

        assert!(SearchAlgorithms::binary_search(test_vec, test_tar, false));
    }

    #[test]
    fn test_salgs_four() {
        let test_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        let test_tar: i32 = 3;
        let expected: usize = 2;

        assert_eq!(
            SearchAlgorithms::binary_search_position(test_vec, test_tar, true),
            expected
        );
    }

    #[test]
    fn test_salgs_five() {
        let test_vec: Vec<i32> = vec![1, 2, 3, 4, 6, 7, 8, 9];
        let test_tar: i32 = 5;
        let expected: usize = 4;

        assert_eq!(
            SearchAlgorithms::binary_search_position(test_vec, test_tar, true),
            expected
        );
    }

    #[test]
    fn test_salgs_six() {
        let test_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        let test_tar: i32 = 3;
        let expected: usize = 2;

        if let Some(res) = SearchAlgorithms::linear_search(&test_vec, test_tar) {
            assert_eq!(expected, res);
        }
    }

    #[test]
    fn test_salgs_seven() {
        let test_vec: [i32; 6] = [2, 5, 7, 8, 12, 16];
        let test_tar: i32 = 8;
        let expected: usize = 3;

        if let Some(res) = SearchAlgorithms::linear_search(&test_vec, test_tar) {
            assert_eq!(expected, res);
        }
    }
}
