use crate::utils::list::*;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1: &Option<Box<ListNode>> = &list1;
        let mut l2: &Option<Box<ListNode>> = &list2;

        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut ptr: &mut Option<Box<ListNode>> = &mut head;

        while l1.is_some() || l2.is_some() {
            match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        ptr.as_mut()?.next = Some(Box::new(ListNode::new(n1.val)));
                        ptr = &mut ptr.as_mut()?.next;

                        l1 = &n1.next;
                    } else {
                        ptr.as_mut()?.next = Some(Box::new(ListNode::new(n2.val)));
                        ptr = &mut ptr.as_mut()?.next;

                        l2 = &n2.next;
                    }
                }

                (Some(node), None) => {
                    ptr.as_mut()?.next = Some(Box::new(ListNode::new(node.val)));
                    ptr = &mut ptr.as_mut()?.next;

                    l1 = &node.next;
                }

                (None, Some(node)) => {
                    ptr.as_mut()?.next = Some(Box::new(ListNode::new(node.val)));
                    ptr = &mut ptr.as_mut()?.next;

                    l2 = &node.next;
                }

                (None, None) => {}
            }
        }

        head?.next
    }
}

#[cfg(test)]
mod tests {
    use crate::list;

    use super::*;

    #[test]
    fn test_021_1() {
        let l1: Option<Box<ListNode>> = list!(1, 3, 5);
        let l2: Option<Box<ListNode>> = list!(2, 4, 6);

        assert_eq!(list!(1, 2, 3, 4, 5, 6), Solution::merge_two_lists(l1, l2));
    }
}
