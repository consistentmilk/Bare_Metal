use crate::utils::tree::*;

use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct Solution;

impl Solution {
    // Iterative solution using inorder traversal
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut prev: Option<i32> = None;
        let mut current: Option<Rc<RefCell<TreeNode>>> = root;

        while current.is_some() || !stack.is_empty() {
            // Go to the leftmost node
            while let Some(node) = current {
                stack.push(Rc::clone(&node));
                current = node.borrow().left.clone();
            }

            // Visit node
            if let Some(node) = stack.pop() {
                let val = node.borrow().val;

                if let Some(p) = prev {
                    if val <= p {
                        return false;
                    }
                }

                prev = Some(val);
                current = node.borrow().right.clone();
            }
        }

        true
    }

    pub fn is_valid_bst_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::validate(root, None, None)
    }

    fn validate(node: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
        match node {
            None => true,

            Some(n) => {
                let n: Ref<'_, TreeNode> = n.borrow();
                let val: i32 = n.val;

                // Check current node's value is within bounds
                if min.is_some() && val <= min.unwrap() {
                    return false;
                }
                if max.is_some() && val >= max.unwrap() {
                    return false;
                }

                // Recursively check left and right subtrees
                Self::validate(n.left.clone(), min, Some(val))
                    && Self::validate(n.right.clone(), Some(val), max)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_both_implementations(root: Option<Rc<RefCell<TreeNode>>>, expected: bool) {
        assert_eq!(
            Solution::is_valid_bst(root.clone()),
            expected,
            "Recursive implementation failed"
        );
        assert_eq!(
            Solution::is_valid_bst_recursive(root),
            expected,
            "Iterative implementation failed"
        );
    }

    #[test]
    fn test_098_1() {
        // Empty tree
        let root = TreeNode::from_vec(vec![]);
        test_both_implementations(root, true);
    }

    #[test]
    fn test_098_2() {
        // Single node
        let root = TreeNode::from_vec(vec![Some(1)]);
        test_both_implementations(root, true);
    }

    #[test]
    fn test_098_3() {
        // Valid BST
        let root = TreeNode::from_vec(vec![Some(2), Some(1), Some(3)]);
        test_both_implementations(root, true);
    }

    #[test]
    fn test_098_4() {
        // Invalid BST (right subtree contains smaller value)
        let root = TreeNode::from_vec(vec![
            Some(5),
            Some(1),
            Some(4),
            None,
            None,
            Some(3),
            Some(6),
        ]);
        test_both_implementations(root, false);
    }

    #[test]
    fn test_098_5() {
        // Large valid BST
        let root = TreeNode::from_vec(vec![
            Some(10),
            Some(5),
            Some(15),
            None,
            None,
            Some(12),
            Some(20),
        ]);
        test_both_implementations(root, true);
    }

    #[test]
    fn test_098_6() {
        // Duplicate values (invalid)
        let root = TreeNode::from_vec(vec![Some(2), Some(2), Some(2)]);
        test_both_implementations(root, false);
    }
}
