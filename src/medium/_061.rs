use crate::utils::list::*;

pub struct Solution;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let k: usize = k as usize;
        let mut ptr: &mut Option<Box<ListNode>> = &mut head;
        let mut list: Vec<i32> = Vec::new();

        while let Some(node) = ptr {
            list.push(node.val);

            ptr = &mut node.next;
        }

        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut ptr: &mut Option<Box<ListNode>> = &mut head;
        let n: usize = list.len();
        let k: usize = n - (k % n);

        if k == 0 {
            for i in k..n {
                let node: Option<Box<ListNode>> = Some(Box::new(ListNode::new(list[i])));
                ptr.as_mut()?.next = node;
                ptr = &mut ptr.as_mut()?.next;
            }

            return head?.next;
        }

        for i in k..n {
            let node: Option<Box<ListNode>> = Some(Box::new(ListNode::new(list[i])));
            ptr.as_mut()?.next = node;
            ptr = &mut ptr.as_mut()?.next;
        }

        for i in 0..k {
            let node: Option<Box<ListNode>> = Some(Box::new(ListNode::new(list[i])));
            ptr.as_mut()?.next = node;
            ptr = &mut ptr.as_mut()?.next;
        }

        head?.next
    }
}

pub struct SolutionAltOne;

impl SolutionAltOne {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Edge cases: empty list or single-node list.
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        // First, compute the length of the list and get a mutable pointer to the tail.
        let mut len: usize = 1;
        {
            let mut p: &mut Box<ListNode> = head.as_mut().unwrap();
            while let Some(ref mut next_node) = p.next {
                p = next_node;
                len += 1;
            }
        }

        // Compute the effective rotations needed.
        let k: usize = (k as usize) % len;
        if k == 0 {
            return head;
        }

        // Now, traverse again to get the tail as a raw pointer.
        let tail: *mut Box<ListNode> = {
            let mut p: &mut Box<ListNode> = head.as_mut().unwrap();
            while p.next.is_some() {
                p = p.next.as_mut().unwrap();
            }
            p as *mut Box<ListNode>
        };

        // Form a circular list: set tail.next = head.
        // Using unsafe to break the borrow so we can move head.
        unsafe {
            (*tail).next = head;
        }

        // To get the new tail, we need to move (len - k) steps from the beginning.
        let steps_to_new_tail: usize = len - k;
        // Start at the (old) head (which is now accessible via (*tail).next).
        let mut new_tail: *mut Box<ListNode> =
            unsafe { (*tail).next.as_mut().unwrap() as *mut Box<ListNode> };
        for _ in 0..steps_to_new_tail - 1 {
            unsafe {
                new_tail = (*new_tail).next.as_mut().unwrap() as *mut Box<ListNode>;
            }
        }

        // The new head is the node right after new_tail.
        // Break the circle by setting new_tail.next to None.
        unsafe {
            let new_head: Option<Box<ListNode>> = (*new_tail).next.take();
            new_head
        }
    }
}

pub struct SolutionAltTwo;

impl SolutionAltTwo {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_61_1() {
        let head: Option<Box<ListNode>> = list!(1, 2, 3, 4, 5);
        let k: i32 = 2;
        let expected: Vec<i32> = vec![4, 5, 1, 2, 3];
        let res: Vec<i32> = ListNode::list_to_vec(Solution::rotate_right(head, k));

        assert_eq!(expected, res);
    }

    #[test]
    fn test_61_2() {
        let head: Option<Box<ListNode>> = list!(0, 1, 2);
        let k: i32 = 4;
        let expected: Vec<i32> = vec![2, 0, 1];
        let res: Vec<i32> = ListNode::list_to_vec(Solution::rotate_right(head, k));

        assert_eq!(expected, res);
    }

    #[test]
    fn test_61_3() {
        let head: Option<Box<ListNode>> = list!(2, 3, 5, 7, 11);
        let k: i32 = 5;
        let expected: Vec<i32> = vec![2, 3, 5, 7, 11];
        let res: Vec<i32> = ListNode::list_to_vec(Solution::rotate_right(head, k));

        assert_eq!(expected, res);
    }

    #[test]
    fn test_61_4() {
        let head: Option<Box<ListNode>> = list!(2, 3, 5, 7, 11);
        let k: i32 = 0;
        let expected: Vec<i32> = vec![2, 3, 5, 7, 11];
        let res: Vec<i32> = ListNode::list_to_vec(Solution::rotate_right(head, k));

        assert_eq!(expected, res);
    }
}
