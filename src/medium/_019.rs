use crate::utils::list::*;

pub struct Solution;

impl Solution {
    ///
    /// S: Slow Pointer | F: Fast Pointer
    /// dummy(S, F) -> 1 -> 2 -> 3 -> 4 -> 5 | Remove 2nd node from last.
    /// dummy(S) -> 1 -> 2(F) -> 3 -> 4 -> 5
    /// dummy -> 1(S) -> 2 -> 3(F) -> 4 -> 5
    /// dummy -> 1 -> 2(S) -> 3 -> 4(F) -> 5
    /// dummy -> 1 -> 2 -> 3(S) -> 4 -> 5(F) | Iteration stops here
    ///
    /// 'S' stops at the node before the one we want to remove.
    /// So in this case, we just have to change the 'next' pointer of 'S'
    /// to one node after the current 'next' node to remove the required node.
    ///
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head_ptr: Box<ListNode> = Box::new(ListNode { val: 0, next: head });
        let mut slow_ptr: &mut Box<ListNode> = &mut head_ptr;
        let mut fast_ptr: &Box<ListNode> = &slow_ptr.clone();

        for _ in 0..n {
            fast_ptr = fast_ptr.next.as_ref()?;
        }

        while fast_ptr.next.is_some() {
            slow_ptr = slow_ptr.next.as_mut()?;
            fast_ptr = fast_ptr.next.as_ref()?;
        }

        slow_ptr.next = slow_ptr.next.as_mut()?.next.take();

        head_ptr.next
    }

    pub fn remove_nth_from_end_unsafe_miri_tested(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy: Box<ListNode> = Box::new(ListNode { val: 0, next: head });
        let mut fast: *const ListNode = &(*dummy) as *const ListNode;

        for _ in 0..n {
            unsafe {
                fast = (*fast).next.as_deref()?;
            }
        }

        let mut slow: *mut ListNode = &mut (*dummy) as *mut ListNode;
        unsafe {
            while (*fast).next.is_some() {
                fast = (*fast).next.as_deref()?;
                slow = (*slow).next.as_deref_mut()?;
            }

            (*slow).next = (*slow).next.as_mut()?.next.take();
        }

        dummy.next
    }

    pub fn remove_nth_from_end_naive(
        mut head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut v: Vec<Option<Box<ListNode>>> = vec![];

        while let Some(mut node) = head {
            head = node.next.take();

            v.push(Some(node));
        }

        let mut expected: Option<Box<ListNode>> = None;

        for (i, link) in v.into_iter().rev().enumerate() {
            if i != (n - 1) as usize {
                let mut node: Box<ListNode> = link.unwrap();

                node.next = expected;

                expected = Some(node);
            }
        }

        expected
    }

    pub fn remove_nth_from_end_alt(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn recurse(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, usize) {
            match head {
                None => (None, 1),

                Some(mut node) => {
                    let (prev, num) = recurse(node.next.take(), n);

                    if n == num as i32 {
                        (prev, num + 1)
                    } else {
                        node.next = prev;
                        (Some(node), num + 1)
                    }
                }
            }
        }

        recurse(head, n).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::list;

    fn test_all_impls(head: Option<Box<ListNode>>, n: i32, expected: Option<Box<ListNode>>) {
        assert_eq!(
            Solution::remove_nth_from_end(head.clone(), n.clone()),
            expected.clone()
        );

        assert_eq!(
            Solution::remove_nth_from_end_unsafe_miri_tested(head.clone(), n.clone()),
            expected.clone()
        );

        assert_eq!(
            Solution::remove_nth_from_end_naive(head.clone(), n.clone()),
            expected.clone()
        );

        assert_eq!(
            Solution::remove_nth_from_end_alt(head.clone(), n.clone()),
            expected.clone()
        );
    }

    #[test]
    fn test_019_1() {
        let head: Option<Box<ListNode>> = list!(1, 2, 3, 4, 5);
        let n: i32 = 1;
        let expected: Option<Box<ListNode>> = list!(1, 2, 3, 4);

        test_all_impls(head, n, expected);
    }

    #[test]
    fn test_019_2() {
        let head: Option<Box<ListNode>> = list!(1);
        let n: i32 = 1;
        let expected: Option<Box<ListNode>> = None;

        test_all_impls(head, n, expected);
    }

    #[test]
    fn test_019_3() {
        let head: Option<Box<ListNode>> = list!(1, 2);
        let n: i32 = 1;
        let expected: Option<Box<ListNode>> = list!(1);

        test_all_impls(head, n, expected);
    }

    #[test]
    fn test_019_4() {
        // Remove first node (n = length)
        let head = list!(1, 2, 3, 4, 5);
        let n = 5;
        let expected = list!(2, 3, 4, 5);

        test_all_impls(head, n, expected);
    }

    #[test]
    fn test_019_5() {
        // Remove middle node
        let head = list!(1, 2, 3, 4, 5);
        let n = 3;
        let expected = list!(1, 2, 4, 5);

        test_all_impls(head, n, expected);
    }

    #[test]
    fn test_019_6() {
        // Single node, remove first (n=1)
        let head = list!(1);
        let n = 1;
        let expected: Option<Box<ListNode>> = None;

        test_all_impls(head, n, expected);
    }

    #[test]
    fn test_019_7() {
        // Two nodes, remove first (n=2)
        let head = list!(1, 2);
        let n = 2;
        let expected = list!(2);

        test_all_impls(head, n, expected);
    }

    #[test]
    fn test_019_8() {
        // Long list, remove 2nd from end
        let head = list!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        let n = 2;
        let expected = list!(1, 2, 3, 4, 5, 6, 7, 8, 10);

        test_all_impls(head, n, expected);
    }

    #[test]
    fn test_019_9() {
        // All nodes same value
        let head = list!(5, 5, 5, 5, 5);
        let n = 2;
        let expected = list!(5, 5, 5, 5);

        test_all_impls(head, n, expected);
    }

    #[test]
    fn test_019_10() {
        // Remove last node in long list
        let head = list!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        let n = 1;
        let expected = list!(1, 2, 3, 4, 5, 6, 7, 8, 9);

        test_all_impls(head, n, expected);
    }
}
