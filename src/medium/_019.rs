use crate::utils::list::*;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
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
    fn test_one() {
        let head: Option<Box<ListNode>> = list![1, 2, 3, 4, 5];
        let res: Option<Box<ListNode>> = list![1, 2, 3, 4];
        let n: i32 = 1;

        assert_eq!(Solution::remove_nth_from_end_alt(head, n), res);
    }
}
