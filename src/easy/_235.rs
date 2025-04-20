//! # Lowest Common Ancestor of a BST (Reference using `cmp`/`match`)
//!
//! ## Intuition
//! 1. Extract the integer values of `p` and `q` once.
//! 2. Traverse from the root, using BST ordering to guide the descent.
//! 3. Use a single `match` on `(p_val.cmp(&node_val), q_val.cmp(&node_val))`.
//! 4. When the two orderings split, the current node is the LCA.
//!
//! ## Complexity
//! - Time: O(h) where h = tree height (balanced ~O(log n), worst-case O(n)).
//! - Space: O(1) extra (only a few pointers and integers).

use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::rc::Rc;

use crate::utils::tree::*;

pub struct Solution;

impl Solution {
    /// Finds the lowest common ancestor (LCA) of two nodes in a BST.
    ///
    /// # Arguments
    ///
    /// * `root` - The root of the BST (ownership moved).
    /// * `p` - First target node (ownership moved).
    /// * `q` - Second target node (ownership moved).
    ///
    /// # Returns
    ///
    /// The LCA node if found, else `None`.
    #[inline(always)]
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Extract the integer values of p and q once at the start.
        let p_val: i32 = p.unwrap().borrow().val;
        let q_val: i32 = q.unwrap().borrow().val;

        // Traverse downward until we either exhaust the tree or find the split point.
        while let Some(node_rc) = root {
            // Borrow the node a single time for this iteration.
            let node_ref: Ref<'_, TreeNode> = node_rc.borrow();

            // Cache the current node's value for both comparisons.
            let node_val: i32 = node_ref.val;

            // Compare (p_val vs node_val) and (q_val vs node_val) together.
            match (p_val.cmp(&node_val), q_val.cmp(&node_val)) {
                // Both targets are greater: move to the right subtree.
                (Ordering::Greater, Ordering::Greater) => {
                    root = node_ref.right.clone();
                }

                // Both targets are less: move to the left subtree.
                (Ordering::Less, Ordering::Less) => {
                    root = node_ref.left.clone();
                }

                // Otherwise, we've found the split point: return this node.
                _ => {
                    return Some(node_rc.clone());
                }
            }
        }

        // Unreachable if p and q are guaranteed to exist in the tree.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_235_lca_root_split() {
        // Build BST [6,2,8,0,4,7,9,null,null,3,5]
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        // Get a strong reference to the root node
        let root_rc = root.as_ref().unwrap().clone();
        // Select node p = 2 (left child of root)
        let p = root_rc.borrow().left.clone();
        // Select node q = 8 (right child of root)
        let q = root_rc.borrow().right.clone();
        // Compute the LCA of p and q
        let lca = Solution::lowest_common_ancestor(root.clone(), p, q);
        // Expect the LCA value to be 6
        assert_eq!(lca.unwrap().borrow().val, 6);
    }

    #[test]
    fn test_235_lca_same_subtree() {
        // Build the same BST [6,2,8,0,4,7,9,null,null,3,5]
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        // Reference the root
        let root_rc = root.as_ref().unwrap().clone();
        // Select node p = 2 (left child of root)
        let left = root_rc.borrow().left.clone().unwrap();
        // Select node q = 4 (right child of node 2)
        let q = left.borrow().right.clone();
        // Compute the LCA (both in left subtree)
        let lca = Solution::lowest_common_ancestor(root.clone(), Some(left.clone()), q.clone());
        // Expect the LCA value to be 2
        assert_eq!(lca.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_235_lca_descend_left() {
        // Build the same BST [6,2,8,0,4,7,9,null,null,3,5]
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        // Reference the root
        let root_rc = root.as_ref().unwrap().clone();
        // Select node p = 0 (left child of node 2)
        let left = root_rc.borrow().left.clone().unwrap();
        let p = left.borrow().left.clone();
        // Select node q = 5 (right-right child under node 2)
        let q = left.borrow().right.clone().unwrap().borrow().right.clone();
        // Compute the LCA
        let lca = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        // Expect the LCA value to be 2
        assert_eq!(lca.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_235_lca_descend_right() {
        // Build the same BST [6,2,8,0,4,7,9,null,null,3,5]
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        // Reference the root
        let root_rc = root.as_ref().unwrap().clone();
        // Select node p = 7 (left child of node 8)
        let right = root_rc.borrow().right.clone().unwrap();
        let p = right.borrow().left.clone();
        // Select node q = 9 (right child of node 8)
        let q = right.borrow().right.clone();
        // Compute the LCA
        let lca = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        // Expect the LCA value to be 8
        assert_eq!(lca.unwrap().borrow().val, 8);
    }

    #[test]
    fn test_235_lca_sibling_nodes() {
        // Build the same BST [6,2,8,0,4,7,9,null,null,3,5]
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        // Reference node 4's parent
        let node4 = root
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap();
        // Select node p = 3 (left child of node 4)
        let p = node4.borrow().left.clone();
        // Select node q = 5 (right child of node 4)
        let q = node4.borrow().right.clone();
        // Compute the LCA
        let lca = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        // Expect the LCA value to be 4
        assert_eq!(lca.unwrap().borrow().val, 4);
    }

    #[test]
    fn test_235_lca_leaf_nodes() {
        // Build the same BST [6,2,8,0,4,7,9,null,null,3,5]
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        // Reference the left subtree root
        let left = root.as_ref().unwrap().borrow().left.clone().unwrap();
        // Select node p = 3
        let p = left.borrow().right.clone().unwrap().borrow().left.clone();
        // Select node q = 0
        let q = left.borrow().left.clone();
        // Compute the LCA
        let lca = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        // Expect the LCA value to be 2
        assert_eq!(lca.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_235_lca_deep_nodes() {
        // Build BST [5,3,6,2,4,null,null,1]
        let root = TreeNode::from_vec(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ]);
        // Reference the root
        let root_rc = root.as_ref().unwrap().clone();
        // Navigate to node 2
        let left1 = root_rc.borrow().left.clone().unwrap();
        let left2 = left1.borrow().left.clone().unwrap();
        // Select node p = 1 (left child of node 2)
        let p = left2.borrow().left.clone();
        // Select node q = 4 (right child of node 3)
        let q = left1.borrow().right.clone();
        // Compute the LCA
        let lca = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        // Expect the LCA value to be 3
        assert_eq!(lca.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_235_lca_skewed_right_tree() {
        // Build right-skewed BST [1,null,2,null,3]
        let root = TreeNode::from_vec(vec![Some(1), None, Some(2), None, Some(3)]);
        // Reference the root
        let root_rc = root.as_ref().unwrap().clone();
        // Select node p = 2 (right child of root)
        let p = root_rc.borrow().right.clone();
        // Select node q = 3 (right child of node 2)
        let q = root_rc
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone();
        // Compute the LCA
        let lca = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        // Expect the LCA value to be 2
        assert_eq!(lca.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_235_lca_skewed_left_tree() {
        // Build left-skewed BST [3,2,null,1,null]
        let root = TreeNode::from_vec(vec![Some(3), Some(2), None, Some(1), None]);
        // Reference the root
        let root_rc = root.as_ref().unwrap().clone();
        // Select node p = 1 (left child of node 2)
        let p = root_rc.borrow().left.clone().unwrap().borrow().left.clone();
        // Select node q = 2 (left child of root)
        let q = root_rc.borrow().left.clone();
        // Compute the LCA
        let lca = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        // Expect the LCA value to be 2
        assert_eq!(lca.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_235_lca_different_subtree() {
        // Build BST [5,3,6,2,4,null,null,1]
        let root = TreeNode::from_vec(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ]);
        // Reference the root
        let root_rc = root.as_ref().unwrap().clone();
        // Select node p = 1 (deep left)
        let left1 = root_rc.borrow().left.clone().unwrap();
        let left2 = left1.borrow().left.clone().unwrap();
        let p = left2.borrow().left.clone();
        // Select node q = 6 (right child of root)
        let q = root_rc.borrow().right.clone();
        // Compute the LCA
        let lca = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        // Expect the LCA value to be 5
        assert_eq!(lca.unwrap().borrow().val, 5);
    }
}
