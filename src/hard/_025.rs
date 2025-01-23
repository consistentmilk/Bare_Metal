use crate::utils::list::*;

pub struct Solution {}

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = head;
        let mut node: &mut Option<Box<ListNode>> = &mut head;

        for _ in 0..k {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                return head;
            }
        }

        let mut ret: Option<Box<ListNode>> = Self::reverse_k_group(node.take(), k);

        while let Some(h) = head.take() {
            ret = Some(Box::new(ListNode {
                val: h.val,
                next: ret,
            }));
            head = h.next;
        }

        ret
    }
}
