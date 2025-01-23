use crate::utils::list::*;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution {}

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::with_capacity(lists.len());

        for list in lists {
            match list {
                Some(node) => heap.push(node),
                None => {}
            }
        }

        let mut dummy_node: Box<ListNode> = Box::new(ListNode::new(0));

        let mut curr_node: &mut Box<ListNode> = &mut dummy_node;

        while let Some(node) = heap.pop() {
            let new_node: Box<ListNode> = Box::new(ListNode::new(node.val));
            curr_node.next = Some(new_node);
            curr_node = curr_node.next.as_mut().unwrap();

            if node.next.is_some() {
                heap.push(node.next.unwrap());
            }
        }

        return dummy_node.next;
    }
}
