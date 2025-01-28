use crate::utils::list::*;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_206_1() {
        let l1: Option<Box<ListNode>> = list!(1, 2, 3, 4, 5);
        assert_eq!(
            ListNode::list_to_vec(Solution::reverse_list(l1)),
            vec![5, 4, 3, 2, 1]
        );
    }

    #[test]
    fn test_206_2() {
        let l1: Option<Box<ListNode>> = None;
        assert_eq!(Solution::reverse_list(l1), None);
    }
}
