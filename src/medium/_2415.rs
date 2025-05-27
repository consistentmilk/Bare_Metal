//! ===============================================================
//!  Leetcode 2415 – Reverse Odd Levels of Binary Tree
//! ===============================================================
//!
//!  Intuition
//!  ---------
//!  A perfect binary tree is *symmetric*: every node on the left
//!  subtree has a mirror counterpart on the right.  To reverse the
//!  values at each **odd** level we do **paired DFS**:
//!
//!  * visit the left-child of a node together with its symmetric
//!    right-child;
//!  * if the current level is odd, swap their `val` fields;
//!  * recurse on the next pair of grandchildren in mirrored order.
//!
//!  Because the tree is perfect, both grandchildren pairs are
//!  guaranteed to exist until the leaves are reached.
//!
//!  Algorithm
//!  ---------
//!  1.  Start at the root.  Its children form the first odd level
//!      (level 1).  
//!  2.  `dfs_mirror(left, right, level)`
//!      * **If** `level` is odd → swap `left.val` and `right.val`.
//!      * Recurse on `(left.left, right.right)` with `level+1`.
//!      * Recurse on `(left.right, right.left)` with `level+1`.
//!  3.  The traversal touches every node exactly once.
//!
//!  Time Complexity
//!  ---------------
//!  * `Θ(n)` – each of the *n* nodes is visited once.
//!
//!  Space Complexity
//!  ----------------
//!  * `Θ(h)`  – recursion depth equals tree height  
//!    (`h = log₂(n)` for a perfect tree).  No auxiliary heaps,
//!    vectors or queues are allocated.
//!
//!  -----------------------------------------------------------------

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::utils::tree::*;

pub struct Solution;

impl Solution {
    /// Entrypoint – reverses node values on each odd tree level
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //  Early-exit: root is None → nothing to do
        if root.is_none() {
            return root;
        }

        //  Safe to unwrap – checked above
        let node: &Rc<RefCell<TreeNode>> = root.as_ref().unwrap();

        //  A perfect tree guarantees both children either exist
        //  together or not at all
        if let (Some(left), Some(right)) = (&node.borrow().left, &node.borrow().right) {
            //  Start mirror-DFS at level 1 (first odd level)
            Self::dfs_mirror(left.clone(), right.clone(), 1);
        }

        //  Return mutated tree
        root
    }

    /// ----------------------------------------------------------
    ///  Recursively visits symmetric node pairs
    ///  * `left`  – node on the left subtree
    ///  * `right` – mirror node on the right subtree
    ///  * `level` – depth counted from the root (root = 0)
    /// ----------------------------------------------------------
    fn dfs_mirror(left: Rc<RefCell<TreeNode>>, right: Rc<RefCell<TreeNode>>, level: usize) {
        //  -------- Swap phase (odd levels only) ----------------
        if level % 2 == 1 {
            //  Obtain mutable borrows of both nodes
            let mut left_mut: RefMut<'_, TreeNode> = left.borrow_mut();
            let mut right_mut: RefMut<'_, TreeNode> = right.borrow_mut();

            //  Exchange the values in-place
            std::mem::swap(&mut left_mut.val, &mut right_mut.val);
            //  Mutable borrows end here when `left_mut`/`right_mut`
            //  go out of scope
        }

        //  -------- Prepare grandchildren -----------------------
        //  Gather symmetric grandchildren while holding only
        //  immutable borrows (does not conflict with past
        //  mutable borrows).
        let (l_left, l_right, r_left, r_right) = {
            let left_ref: Ref<'_, TreeNode> = left.borrow();
            let right_ref: Ref<'_, TreeNode> = right.borrow();

            (
                left_ref.left.clone(),
                left_ref.right.clone(),
                right_ref.left.clone(),
                right_ref.right.clone(),
            )
        };

        //  -------- Recurse on first symmetric pair --------------
        if let (Some(ll), Some(rr)) = (l_left, r_right) {
            Self::dfs_mirror(ll, rr, level + 1);
        }

        //  -------- Recurse on second symmetric pair -------------
        if let (Some(lr), Some(rl)) = (l_right, r_left) {
            Self::dfs_mirror(lr, rl, level + 1);
        }
    }
}
