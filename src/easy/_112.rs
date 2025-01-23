use crate::utils::tree::*;

use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(n) = root {
            let node: Ref<TreeNode> = n.borrow();

            if node.left.is_none() && node.right.is_none() && node.val == sum {
                return true;
            }

            return Solution::has_path_sum(node.left.clone(), sum - node.val)
                || Solution::has_path_sum(node.right.clone(), sum - node.val);
        }

        false
    }
}
