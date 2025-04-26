use crate::utils::list::*;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Box<ListNode> = Box::new(ListNode::new(0));
        let mut ptr: &mut Box<ListNode> = &mut head;
        let mut carry: i32 = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut total: i32 = carry;

            if let Some(node) = l1 {
                total += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                total += node.val;
                l2 = node.next;
            }

            ptr.next = Some(Box::new(ListNode::new(total % 10)));
            ptr = ptr.next.as_mut()?;

            carry = total / 10;
        }

        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_002_1() {
        let l1: Option<Box<ListNode>> = ListNode::list_maker(vec![2, 4, 3]);
        let l2: Option<Box<ListNode>> = ListNode::list_maker(vec![5, 6, 4]);

        let res: Vec<i32> = ListNode::list_extract(Solution::add_two_numbers(l1, l2));

        let expected: Vec<i32> = vec![7, 0, 8];

        assert_eq!(res, expected);
    }

    #[test]
    fn test_002_2() {
        let l1: Option<Box<ListNode>> = ListNode::list_maker(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2: Option<Box<ListNode>> = ListNode::list_maker(vec![9, 9, 9, 9]);

        let res: Vec<i32> = ListNode::list_extract(Solution::add_two_numbers(l1, l2));
        let expected: Vec<i32> = vec![8, 9, 9, 9, 0, 0, 0, 1];

        assert_eq!(res, expected);
    }
}
