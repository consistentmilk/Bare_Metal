/*
Intuition:
1. Inorder traversal follows the LEFT → ROOT → RIGHT visitation order.
2. Recursion naturally leverages the call stack to traverse subtrees in that order.
3. An explicit stack can simulate that call stack for an iterative solution.
4. Both approaches visit each node exactly once in the correct sequence.

Algorithm:
- Recursive version:
  1. If the current node is None, do nothing.
  2. Recursively traverse the left subtree.
  3. Visit the current node by appending its value to the result vector.
  4. Recursively traverse the right subtree.

- Iterative version:
  1. Initialize an empty stack and set `curr` to the root.
  2. While `curr` is not None or the stack is not empty:
     a. Drill down to the leftmost node: push each visited node onto the stack and move `curr` to its left child.
     b. Once you reach a None, pop the top node from the stack.
     c. Visit that node by appending its value to the result vector.
     d. Set `curr` to the right child of the popped node and repeat.

Time Complexity:
- O(n), where n is the number of nodes, since each node is processed exactly once.

Space Complexity:
- Recursive: O(h) due to call stack, where h is the height of the tree.
- Iterative: O(h) due to the explicit stack, where h is the height of the tree.
*/

use std::cell::RefCell;
// The `RefCell` type allows mutable borrows checked at runtime.
use std::rc::Rc;
// `Rc` provides shared ownership of tree nodes.

use crate::utils::tree::*;
// Import the `TreeNode` definition and related utilities.

pub struct Solution;
// Empty struct to hold the solution methods.

impl Solution {
    // LEFT → ROOT → RIGHT recursive inorder traversal.
    // Suppress warnings if this method goes unused.
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Initialize the result vector to collect node values.
        let mut res_stack: Vec<i32> = Vec::new();

        // Kick off the recursive helper to fill `res_stack`.
        Self::traverse(root, &mut res_stack);

        // Return the populated result vector.
        res_stack
    }

    // Helper function for the recursive inorder traversal.
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, res_stack: &mut Vec<i32>) {
        // If the current node exists, proceed; otherwise, return immediately.
        if let Some(node) = root {
            // Recurse into the left subtree first.
            Self::traverse(node.borrow().left.clone(), res_stack);

            // Visit the node: extract its value and append to `res_stack`.
            res_stack.push(node.borrow().val);

            // Recurse into the right subtree.
            Self::traverse(node.borrow().right.clone(), res_stack);
        }
    }

    // Iterative inorder traversal using an explicit stack.
    // Suppress warnings if this method goes unused.
    pub fn inorder_traversal_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Result vector to collect node values.
        let mut res_stack: Vec<i32> = Vec::new();

        // Stack to simulate the recursion call stack.
        let mut node_stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        // Start traversal at the root.
        let mut current: Option<Rc<RefCell<TreeNode>>> = root;

        // Continue until there are no nodes left to visit.
        while current.is_some() || !node_stack.is_empty() {
            // Travel to the leftmost node.
            while let Some(node) = current {
                // Push the current node onto the stack.
                node_stack.push(node.clone());

                // Move `curr` to the left child.
                current = node.borrow().left.clone();
            }

            // Pop the top node from the stack once leftmost is reached.
            if let Some(node) = node_stack.pop() {
                // Visit the node: append its value to `res_stack`.
                res_stack.push(node.borrow().val);

                // Switch to the right subtree of the popped node.
                current = node.borrow().right.clone();
            }
        }

        // Return the collected inorder sequence.
        res_stack
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
