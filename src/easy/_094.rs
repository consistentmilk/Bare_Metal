use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::*;
pub struct Solution;

impl Solution {
    // LEFT - ROOT - RIGHT
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res_vec: Vec<i32> = Vec::new();

        Self::traverse(root, &mut res_vec);

        res_vec
    }

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, res_vec: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::traverse(node.borrow().left.clone(), res_vec);

            res_vec.push(node.borrow().val);

            Self::traverse(node.borrow().right.clone(), res_vec);
        }
    }

    pub fn inorder_traversal_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res_vec: Vec<i32> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut curr = root;

        while curr.is_some() || !stack.is_empty() {
            while let Some(node) = curr {
                stack.push(node.clone());

                curr = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                res_vec.push(node.borrow().val);

                curr = node.borrow().right.clone();
            }
        }

        res_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_both_implementations(root: Option<Rc<RefCell<TreeNode>>>, expected: Vec<i32>) {
        // Test recursive implementation
        assert_eq!(
            Solution::inorder_traversal(root.clone()),
            expected,
            "Recursive implementation failed"
        );

        // Test iterative implementation
        assert_eq!(
            Solution::inorder_traversal_iterative(root),
            expected,
            "Iterative implementation failed"
        );
    }

    #[test]
    fn test_094_1() {
        let root: Option<Rc<RefCell<TreeNode>>> =
            TreeNode::from_vec(vec![Some(1), None, Some(2), Some(3)]);

        let expected: Vec<i32> = vec![1, 3, 2];

        test_both_implementations(root, expected);
    }

    #[test]
    fn test_094_2() {
        let root: Option<Rc<RefCell<TreeNode>>> =
            TreeNode::from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);

        let expected: Vec<i32> = vec![4, 2, 5, 1, 3];

        test_both_implementations(root, expected);
    }

    #[test]
    fn test_094_3() {
        let root: Option<Rc<RefCell<TreeNode>>> = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);

        let expected: Vec<i32> = vec![9, 3, 15, 20, 7];

        test_both_implementations(root, expected);
    }
}
