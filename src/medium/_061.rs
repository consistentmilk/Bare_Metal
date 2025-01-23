pub struct Solution;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut ptr: Option<&Box<ListNode>> = head.as_ref();
        let mut n: usize = 0;

        while let Some(node) = ptr {
            ptr = node.next.as_ref();
            n += 1;
        }

        if n < 2 {
            return head;
        }

        let k: usize = k as usize % n;

        if k == 0 {
            return head;
        }

        let mut i: usize = 0;
        let mut ptr: Option<&mut Box<ListNode>> = head.as_mut();
        let mut new_head: Option<Box<ListNode>> = None;

        while let Some(node) = ptr {
            if i + k == n - 1 {
                new_head = node.next.take();
                break;
            } else {
                ptr = node.next.as_mut();
            }

            i += 1;
        }

        let mut ptr: Option<&mut Box<ListNode>> = new_head.as_mut();

        while let Some(node) = ptr {
            if node.next.is_none() {
                node.next = head;
                break;
            }

            ptr = node.next.as_mut();
        }

        new_head
    }
}
