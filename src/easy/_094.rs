use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::*;
pub struct Solution;

impl Solution {
    // LEFT - ROOT - RIGHT
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(node) = root {
                traverse(node.borrow().left.clone(), res);

                res.push(node.borrow().val);

                traverse(node.borrow().right.clone(), res);
            }
        }

        traverse(root, &mut res);

        res
    }

    pub fn inorder_traversal_iterative(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                stack.push(node.clone());
                root = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                res.push(node.borrow().val);
                root = node.borrow().right.clone();
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_094_1() {
        let root: Option<Rc<RefCell<TreeNode>>> =
            TreeNode::from_vec(vec![Some(1), None, Some(2), Some(3)]);

        let expected: Vec<i32> = vec![1, 3, 2];

        assert_eq!(Solution::inorder_traversal_iterative(root), expected);
    }

    #[test]
    fn test_094_2() {
        let root: Option<Rc<RefCell<TreeNode>>> =
            TreeNode::from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);

        let expected: Vec<i32> = vec![4, 2, 5, 1, 3];

        assert_eq!(Solution::inorder_traversal_iterative(root), expected);
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

        assert_eq!(Solution::inorder_traversal_iterative(root), expected);
    }
}
