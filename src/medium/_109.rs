use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::list::*;
use crate::utils::tree::*;

pub struct Solution {}

// Input: head = [-10,-3,0,5,9]
// Output: [0,-3,9,-10,null,5]
// Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut len: usize = 0;
        let mut ptr: &Option<Box<ListNode>> = &head;

        while let Some(node) = ptr {
            len += 1;
            ptr = &node.next;
        }

        let mut head = head;

        Self::recurse(&mut head, len)
    }

    fn recurse(list: &mut Option<Box<ListNode>>, len: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if len == 0 {
            return None;
        }

        let left: Option<Rc<RefCell<TreeNode>>> = Self::recurse(list, len / 2);

        if let Some(head) = list {
            let mut node: TreeNode = TreeNode::new(head.val);

            *list = head.next.take();
            node.left = left;
            node.right = Self::recurse(list, len - (len / 2) - 1);

            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }
}
