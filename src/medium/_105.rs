pub struct Solution;

use crate::utils::tree::*;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let root_val: i32 = preorder[0];
        let root_index: usize = inorder.iter().position(|&v| v == root_val)?;

        let left_subtree: Option<Rc<RefCell<TreeNode>>> = Self::build_tree(
            preorder[1..root_index + 1].to_vec(),
            inorder[..root_index].to_vec(),
        );
        let right_subtree: Option<Rc<RefCell<TreeNode>>> = Self::build_tree(
            preorder[root_index + 1..].to_vec(),
            inorder[root_index + 1..].to_vec(),
        );

        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(root_val)));
        root.borrow_mut().left = left_subtree;
        root.borrow_mut().right = right_subtree;

        Some(root)
    }

    pub fn build_tree_alt(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_rec(&preorder, &inorder)
    }

    fn build_tree_rec(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }

        let root_val: i32 = preorder[0];
        let i: usize = inorder.iter().position(|&x| x == root_val).unwrap();
        let l: Option<Rc<RefCell<TreeNode>>> =
            Self::build_tree_rec(&preorder[1..i + 1], &inorder[..i]);
        let r: Option<Rc<RefCell<TreeNode>>> =
            Self::build_tree_rec(&preorder[i + 1..], &inorder[i + 1..]);

        let t: TreeNode = TreeNode {
            val: root_val,
            left: l,
            right: r,
        };

        Some(Rc::new(RefCell::new(t)))
    }
}
