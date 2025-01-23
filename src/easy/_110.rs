use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::*;

pub struct Solution {}

impl Solution {
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(rc_refc_tn) => {
                let left: i32 = Self::helper(&rc_refc_tn.borrow().left);
                let right: i32 = Self::helper(&rc_refc_tn.borrow().right);

                if left == -1 || right == -1 {
                    return -1;
                } else {
                    if i32::abs(left - right) > 1 {
                        return -1;
                    } else {
                        return 1 + i32::max(left, right);
                    }
                }
            }
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::helper(&root) != -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_110_one() {
        let test_root: Option<Rc<RefCell<TreeNode>>> =
            tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));

        assert!(Solution::is_balanced(test_root));
    }

    #[test]
    fn test_110_two() {
        let test_root: Option<Rc<RefCell<TreeNode>>> = None;

        assert!(Solution::is_balanced(test_root));
    }

    #[test]
    fn test_110_three() {
        let test_root: Option<Rc<RefCell<TreeNode>>> = tree!(
            1,
            tree!(2, tree!(3, tree!(4), tree!(4)), tree!(3)),
            tree!(2)
        );

        assert!(!Solution::is_balanced(test_root))
    }
}
