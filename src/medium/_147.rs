use std::collections::BTreeMap;

use crate::utils::list::*;

pub struct Solution;

impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node_counts: BTreeMap<i32, i32> = BTreeMap::new();

        // Count occurrences of each value in the list
        while let Some(node) = head {
            *node_counts.entry(node.val).or_insert(0) += 1;
            head = node.next;
        }

        let mut dummy: Box<ListNode> = Box::new(ListNode::new(0));
        let mut tail: &mut Box<ListNode> = &mut dummy;

        // Reconstruct the sorted list based on counted values
        for (val, count) in node_counts {
            for _ in 0..count {
                tail.next = Some(Box::new(ListNode::new(val)));
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }

    pub fn insertion_sort_list_alt(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        // Step 1: Find the range of the values in the list
        let mut min_val: i32 = i32::MAX;
        let mut max_val: i32 = i32::MIN;
        let mut current: &Option<Box<ListNode>> = &head;

        while let Some(node) = current {
            if node.val < min_val {
                min_val = node.val;
            }
            if node.val > max_val {
                max_val = node.val;
            }
            current = &node.next;
        }

        // Early exit if the list is empty or has only one element
        if min_val == max_val {
            return head;
        }

        // Step 2: Create a count array and populate it
        let range: usize = (max_val - min_val + 1) as usize;
        let mut count: Vec<i32> = vec![0; range];
        current = &head;

        while let Some(node) = current {
            count[(node.val - min_val) as usize] += 1;
            current = &node.next;
        }

        // Step 3: Reconstruct the sorted linked list
        let mut dummy: Box<ListNode> = Box::new(ListNode::new(0));
        let mut tail: &mut Box<ListNode> = &mut dummy;

        for (i, &num) in count.iter().enumerate() {
            for _ in 0..num {
                tail.next = Some(Box::new(ListNode::new(i as i32 + min_val)));
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
