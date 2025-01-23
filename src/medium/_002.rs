#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers_opt(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy: ListNode = ListNode::new(0);
        let (mut p, mut carry) = (&mut dummy, 0);

        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum: i32 = carry
                + l1.as_ref().map_or(0, |node: &Box<ListNode>| node.val)
                + l2.as_ref().map_or(0, |node: &Box<ListNode>| node.val);
            carry = sum / 10;

            p.next = Some(Box::new(ListNode::new(sum % 10)));
            p = p.next.as_mut().unwrap();

            l1 = l1.and_then(|node: Box<ListNode>| node.next);
            l2 = l2.and_then(|node: Box<ListNode>| node.next);
        }

        dummy.next
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut ptr: &mut Option<Box<ListNode>> = &mut head;
        let mut l1: &Option<Box<ListNode>> = &l1;
        let mut l2: &Option<Box<ListNode>> = &l2;
        let mut carry: i32 = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut total = carry;

            if let Some(node) = l1 {
                total += node.val;
                l1 = &node.next;
            }

            if let Some(node) = l2 {
                total += node.val;
                l2 = &node.next;
            }

            ptr.as_mut()?.next = Some(Box::new(ListNode::new(total % 10)));
            ptr = &mut ptr.as_mut()?.next;

            carry = total / 10;
        }

        head?.next
    }

    pub fn list_maker(raw_arr: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: ListNode = ListNode::new(0);
        let mut ptr: &mut ListNode = &mut head;

        for num in raw_arr {
            ptr.next = Some(Box::new(ListNode::new(num)));
            ptr = ptr.next.as_mut().unwrap();
        }

        head.next
    }

    pub fn list_extract(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ptr: Option<Box<ListNode>> = head;
        let mut arr: Vec<i32> = vec![];

        while let Some(node) = ptr.as_ref() {
            arr.push(node.val);
            ptr = node.next.clone();
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_002_1() {
        let l1: Option<Box<ListNode>> = Solution::list_maker(vec![2, 4, 3]);
        let l2: Option<Box<ListNode>> = Solution::list_maker(vec![5, 6, 4]);

        let res: Vec<i32> = Solution::list_extract(Solution::add_two_numbers(l1, l2));

        let expected: Vec<i32> = vec![7, 0, 8];

        assert_eq!(res, expected);
    }

    #[test]
    fn test_002_2() {
        let l1: Option<Box<ListNode>> = Solution::list_maker(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2: Option<Box<ListNode>> = Solution::list_maker(vec![9, 9, 9, 9]);

        let res: Vec<i32> = Solution::list_extract(Solution::add_two_numbers(l1, l2));
        let expected: Vec<i32> = vec![8, 9, 9, 9, 0, 0, 0, 1];

        assert_eq!(res, expected);
    }
}
