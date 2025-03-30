use crate::utils::list::*;

pub struct Solution;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut ptr: &Option<Box<ListNode>> = &head;
        let mut n: i32 = 0;

        while let Some(node) = ptr {
            ptr = &node.next;
            n += 1;
        }

        let k: i32 = k % n;

        if k == 0 {
            return head;
        }

        let mut i: i32 = 0;
        let pivot_point: i32 = n - k;
        let mut ptr: &mut Option<Box<ListNode>> = &mut head;
        let mut pivot: Option<Box<ListNode>> = None;

        while let Some(node) = ptr {
            i += 1;

            if i == pivot_point {
                pivot = node.next.take();
                break;
            }

            ptr = &mut node.next;
        }

        let mut ptr: &mut Option<Box<ListNode>> = &mut pivot;

        while let Some(node) = ptr {
            if node.next.is_none() {
                node.next = head;
                break;
            }

            ptr = &mut node.next;
        }

        pivot
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
