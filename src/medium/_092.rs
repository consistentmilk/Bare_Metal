use crate::utils::list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: head }));

        let mut before: &mut Option<Box<ListNode>> = &mut dummy;
        for _ in 1..left {
            before = &mut before.as_mut()?.next;
        }

        let mut node: Option<Box<ListNode>> = before.as_mut()?.next.take();
        let mut node2: Option<Box<ListNode>> = node.as_mut()?.next.take();
        for _ in left..right {
            let node3: Option<Box<ListNode>> = node2.as_mut()?.next.take();
            node2.as_mut()?.next = node;
            node = node2;
            node2 = node3;
        }

        let mut rev_tail: &mut Option<Box<ListNode>> = &mut node;
        for _ in left..right {
            rev_tail = &mut rev_tail.as_mut()?.next;
        }

        rev_tail.as_mut()?.next = node2;
        before.as_mut()?.next = node;

        dummy.unwrap().next
    }
}
