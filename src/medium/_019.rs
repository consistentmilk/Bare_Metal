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
        let mut dummy: Box<ListNode> = Box::new(ListNode { val: 0, next: head });
        let mut slow_ptr: &mut Box<ListNode> = &mut dummy;
        let mut fast_ptr: &Box<ListNode> = &slow_ptr.clone();

        for _ in 0..n {
            fast_ptr = fast_ptr.next.as_ref()?;
        }

        while fast_ptr.next.is_some() {
            fast_ptr = fast_ptr.next.as_ref()?;
            slow_ptr = slow_ptr.next.as_mut()?;
        }

        slow_ptr.next = slow_ptr.next.as_mut()?.next.take();

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

        let mut res: Option<Box<ListNode>> = None;

        for (i, link) in v.into_iter().rev().enumerate() {
            if i != (n - 1) as usize {
                let mut node: Box<ListNode> = link.unwrap();

                node.next = res;

                res = Some(node);
            }
        }

        res
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

    #[test]
    fn test_19_1() {
        let head: Option<Box<ListNode>> = list!(1, 2, 3, 4, 5);
        let n: i32 = 1;
        let res: Option<Box<ListNode>> = list!(1, 2, 3, 4);

        assert_eq!(Solution::remove_nth_from_end(head, n), res);
    }

    #[test]
    fn test_19_2() {
        let head: Option<Box<ListNode>> = list!(1);
        let n: i32 = 1;
        let res: Option<Box<ListNode>> = None;

        assert_eq!(Solution::remove_nth_from_end(head, n), res);
    }

    #[test]
    fn test_19_3() {
        let head: Option<Box<ListNode>> = list!(1, 2);
        let n: i32 = 1;
        let res: Option<Box<ListNode>> = list!(1);

        assert_eq!(Solution::remove_nth_from_end(head, n), res);
    }
}
