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

    pub fn alt(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut ptr: Option<Box<ListNode>> = head;

        let mut node_container: Vec<Box<ListNode>> = vec![];

        while let Some(mut node) = ptr {
            ptr = node.next.take();

            node_container.push(node);
        }

        let mut modified_head: Option<Box<ListNode>> = None;

        // n is 1-based, but the vector enumeration is 0-based
        let nth: usize = (n - 1) as usize;

        // rust provides double ended iterator for vec, hence we can just use rev
        // with zero overhead instead of working with indices
        for (idx, mut node) in node_container.into_iter().rev().enumerate() {
            if idx != nth {
                node.next = modified_head;

                modified_head = Some(node);
            }
        }

        modified_head
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

        assert_eq!(Solution::alt(head, n), res);
    }
}
