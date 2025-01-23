pub struct BinarySearch;

impl BinarySearch {
    ///
    /// Given an array of i32 type integers, finds its position (usize type index) in the array
    /// and returns it as i32 type.
    ///
    pub fn rank(arr: Vec<i32>, key: i32) -> i32 {
        let mut lp = 0;
        let mut rp = arr.len() - 1;

        while lp <= rp {
            let mid = lp + (rp - lp) / 2;

            if key > arr[mid] {
                lp = mid + 1
            } else if key < arr[mid] {
                rp = mid - 1;
            } else {
                return mid as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algs_binary_search_1() {
        let arr = vec![1, 3, 5, 9, 15];
        let key = 1;
        let expected = 0;

        assert_eq!(BinarySearch::rank(arr, key), expected);
    }

    #[test]
    fn test_algs_binary_search_2() {
        let arr = vec![11, 33, 55, 97, 152, 193, 261];
        let key = 193;
        let expected = 5;

        assert_eq!(BinarySearch::rank(arr, key), expected);
    }
}
