/*
Intuition:
1. A valid BST’s inorder traversal yields a strictly increasing sequence of values.
2. We can check this iteratively by simulating inorder traversal with an explicit stack.
3. Alternatively, recursively enforce that each node’s value stays within valid (min, max) bounds.

Algorithm:
- Iterative (`is_valid_bst`):
  1. Initialize an empty stack and `left_node_val = None`.
  2. While there are nodes to visit:
     a. Drill left: push each node onto the stack and move to its left child.
     b. When no more left children, pop from stack → this is next inorder node.
     c. Compare its value to `left_node_val` (previous). If not greater, return false.
     d. Update `left_node_val` and move to its right child.
  3. If all nodes pass, return true.

- Recursive (`is_valid_bst_recursive` → `validate`):
  1. At each node, check `min < node.val < max`.
  2. Recurse left with updated max = node.val; recurse right with updated min = node.val.
  3. Null nodes return true.

Time Complexity:
- O(n), where n = number of nodes (each visited once).

Space Complexity:
- O(h) stack/recursion depth, where h = tree height (worst case O(n)).
*/

// Import TreeNode and related utilities.
use crate::utils::tree::*;

// RefCell for interior mutability, Ref for borrowed view.
use std::cell::{Ref, RefCell};

// Rc for shared ownership of tree nodes.
use std::rc::Rc;

// Empty struct to namespace the methods.
pub struct Solution;

impl Solution {
    // Iterative inorder traversal to validate BST.
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Stack to hold nodes during traversal.
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        // Holds the value of the last visited node.
        let mut left_node_val: Option<i32> = None;
        // Pointer for the current node, starts at root.
        let mut current: Option<Rc<RefCell<TreeNode>>> = root;

        // Continue until traversal is complete.
        while current.is_some() || !stack.is_empty() {
            // Reach the leftmost node.
            while let Some(node) = current {
                // Push this node to revisit after its left subtree.
                stack.push(Rc::clone(&node));
                // Move to left child.
                current = node.borrow().left.clone();
            }

            // Process the node at the top of the stack.
            if let Some(node) = stack.pop() {
                // Extract its value.
                let right_val: i32 = node.borrow().val;

                // Compare with the previously visited value.
                if let Some(left_val) = left_node_val {
                    // If not strictly increasing, it's invalid.
                    if left_val >= right_val {
                        return false;
                    }
                }

                // Update the previous value to current.
                left_node_val = Some(right_val);
                // Now traverse the right subtree.
                current = node.borrow().right.clone();
            }
        }

        // All nodes satisfied BST property.
        true
    }

    // Recursive bounds‑checking solution.
    pub fn is_valid_bst_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Invoke helper with no initial bounds.
        Self::validate(root, None, None)
    }

    // Helper that verifies all nodes in the subtree are within (min, max).
    fn validate(root: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
        // An empty subtree is valid.
        match root {
            None => true,

            Some(node_rc) => {
                // Borrow the node to access its fields.
                let node: Ref<'_, TreeNode> = node_rc.borrow();
                // Current node’s value.
                let val: i32 = node.val;

                // If a min bound exists and val is not greater, fail.
                if let Some(min_val) = min {
                    if val <= min_val {
                        return false;
                    }
                }
                // If a max bound exists and val is not smaller, fail.
                if let Some(max_val) = max {
                    if val >= max_val {
                        return false;
                    }
                }

                // Recurse left with updated max bound = val,
                // and right with updated min bound = val.
                Self::validate(node.left.clone(), min, Some(val))
                    && Self::validate(node.right.clone(), Some(val), max)
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
