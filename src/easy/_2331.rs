pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::*;

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let val: i32 = node.borrow().val;

                if val == 0 {
                    return false;
                }

                if val == 1 {
                    return true;
                }

                let left_val: bool = Self::evaluate_tree(node.borrow().left.clone());
                let right_val: bool = Self::evaluate_tree(node.borrow().right.clone());

                if val == 2 {
                    return left_val || right_val;
                }

                if val == 3 {
                    return left_val && right_val;
                }

                false
            }

            None => false,
        }
    }
}
