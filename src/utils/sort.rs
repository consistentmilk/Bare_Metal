use std::collections::{binary_heap::PeekMut, BinaryHeap};

pub struct SortAlgs;

struct MergeCandidate<'a, T> {
    first: T,
    rest: &'a [T],
}

impl<'a, T: Copy> From<(&T, &'a [T])> for MergeCandidate<'a, T> {
    fn from((&first, rest): (&T, &'a [T])) -> Self {
        MergeCandidate { first, rest }
    }
}

impl<T: PartialEq> PartialEq for MergeCandidate<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first
    }
}

impl<T: Eq> Eq for MergeCandidate<'_, T> {}

impl<T: PartialOrd> PartialOrd for MergeCandidate<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.first.partial_cmp(&self.first)
    }
}

impl<T: Ord> Ord for MergeCandidate<'_, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.first.cmp(&self.first)
    }
}

impl SortAlgs {
    pub fn bubble_sort<T>(arr: &mut [T])
    where
        T: Copy + PartialOrd,
    {
        let n: usize = arr.len();

        for i in 0..(n - 1) {
            for j in 0..(n - i - 1) {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }

    pub fn selection_sort<T>(arr: &mut [T])
    where
        T: Copy + PartialOrd,
    {
        let n: usize = arr.len();

        for i in 0..n {
            let mut min_index: usize = i;

            for j in (i + 1)..n {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }

            arr.swap(i, min_index);
        }
    }

    pub fn insertion_sort<T>(arr: &mut [T])
    where
        T: Copy + PartialOrd,
    {
        for i in 1..arr.len() {
            let key: T = arr[i];
            let mut j: usize = i;

            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
            }

            arr[j] = key;
        }
    }

    pub fn merge_sort<T>(lists: &[&[T]]) -> Vec<T>
    where
        T: Copy + Ord,
    {
        let mut candidates: BinaryHeap<MergeCandidate<T>> = BinaryHeap::with_capacity(lists.len());
        let mut total_length: usize = 0;

        for &list in lists {
            if let Some(candidate) = list.split_first() {
                candidates.push(MergeCandidate::from(candidate));
                total_length += list.len();
            }
        }

        let mut sorted: Vec<T> = Vec::with_capacity(total_length);

        while let Some(mut merge_candidate) = candidates.peek_mut() {
            sorted.push(merge_candidate.first);

            if let Some(candidate) = merge_candidate.rest.split_first() {
                *merge_candidate = MergeCandidate::from(candidate);
            } else {
                PeekMut::pop(merge_candidate);
            }
        }

        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_one() {
        let mut arr: [i32; 7] = [64, 34, 25, 12, 22, 11, 90];
        let expected: [i32; 7] = [11, 12, 22, 25, 34, 64, 90];

        SortAlgs::bubble_sort(&mut arr);

        assert_eq!(arr, expected);
    }

    #[test]
    fn test_sort_two() {
        let mut arr: [i32; 7] = [64, 34, 25, 12, 22, 11, 90];
        let expected: [i32; 7] = [11, 12, 22, 25, 34, 64, 90];

        SortAlgs::selection_sort(&mut arr);

        assert_eq!(arr, expected);
    }

    #[test]
    fn test_sort_three() {
        let mut arr: [i32; 7] = [64, 34, 25, 12, 22, 11, 90];
        let expected: [i32; 7] = [11, 12, 22, 25, 34, 64, 90];

        SortAlgs::insertion_sort(&mut arr);

        assert_eq!(arr, expected);
    }
}
