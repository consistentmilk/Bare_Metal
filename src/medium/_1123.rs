use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Solution;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn print_tree(root: &Option<Rc<RefCell<TreeNode>>>, prefix: String, is_left: bool) {
        if let Some(node) = root {
            println!(
                "{}{} {}",
                prefix,
                if is_left { "├──" } else { "└──" },
                node.borrow().val
            );

            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            Self::print_tree(&node.borrow().left, new_prefix.clone(), true);
            Self::print_tree(&node.borrow().right, new_prefix, false);
        }
    }
}

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root).0
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        match root {
            None => (None, 0),

            Some(node) => {
                let (left_lca, left_depth) = Self::dfs(&node.borrow().left);
                let (right_lca, right_depth) = Self::dfs(&node.borrow().right);

                match left_depth.cmp(&right_depth) {
                    Ordering::Greater => return (left_lca, left_depth + 1),

                    Ordering::Less => return (right_lca, right_depth + 1),

                    Ordering::Equal => return (Some(Rc::clone(&node)), left_depth + 1),
                }
            }
        }
    }

    pub fn lca_deepest_leaves_iterative(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let mut stack = Vec::new();
        let mut lca_map = HashMap::new(); // Stores (LCA, depth) for each node
        let mut max_depth = 0;
        let root_node = root.unwrap();

        // Post-order traversal stack: (node, visited)
        stack.push((Rc::clone(&root_node), false));

        while let Some((node, visited)) = stack.pop() {
            if !visited {
                // Push back with visited = true
                stack.push((Rc::clone(&node), true));

                // Push right child first (post-order)
                if let Some(right) = node.borrow().right.as_ref() {
                    stack.push((Rc::clone(right), false));
                }

                // Push left child
                if let Some(left) = node.borrow().left.as_ref() {
                    stack.push((Rc::clone(left), false));
                }
            } else {
                // Process the node
                let left_child = node.borrow().left.as_ref().map(Rc::clone);
                let right_child = node.borrow().right.as_ref().map(Rc::clone);

                let (left_lca, left_depth) = left_child
                    .as_ref()
                    .and_then(|lc| lca_map.get(&lc.as_ptr()))
                    .unwrap_or(&(None, 0));

                let (right_lca, right_depth) = right_child
                    .as_ref()
                    .and_then(|rc| lca_map.get(&rc.as_ptr()))
                    .unwrap_or(&(None, 0));

                let (current_lca, current_depth) = match left_depth.cmp(&right_depth) {
                    Ordering::Greater => (left_lca.clone(), left_depth + 1),
                    Ordering::Less => (right_lca.clone(), right_depth + 1),
                    Ordering::Equal => (Some(Rc::clone(&node)), left_depth + 1),
                };

                lca_map.insert(node.as_ptr(), (current_lca, current_depth));
                max_depth = max_depth.max(current_depth);
            }
        }

        // Retrieve the LCA of the root node
        lca_map.get(&root_node.as_ptr()).unwrap().0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefMut;
    use std::collections::VecDeque;

    fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut index: usize = 1;

        while !queue.is_empty() && index < values.len() {
            let node: Rc<RefCell<TreeNode>> = queue.pop_front().unwrap();
            let mut node: RefMut<'_, TreeNode> = node.borrow_mut();

            if index < values.len() {
                if let Some(val) = values[index] {
                    let left: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }

                index += 1;
            }

            if index < values.len() {
                if let Some(val) = values[index] {
                    let right: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }

                index += 1;
            }
        }

        Some(root)
    }

    #[test]
    fn test_1123_1_single_node_tree() {
        // [1]
        let tree = build_tree(&[Some(1)]);
        let result = Solution::lca_deepest_leaves(tree);
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_1123_2_balanced_two_level_tree() {
        //   1
        //  / \
        // 2   3
        let tree = build_tree(&[Some(1), Some(2), Some(3)]);
        let result = Solution::lca_deepest_leaves(tree);
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_1123_3_left_heavy_tree() {
        //     1
        //    /
        //   2
        //  /
        // 3
        let tree = build_tree(&[Some(1), Some(2), None, Some(3), None]);
        let result = Solution::lca_deepest_leaves(tree);
        assert_eq!(result.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_1123_4_right_heavy_tree() {
        // 1
        //  \
        //   2
        //    \
        //     3
        let tree = build_tree(&[Some(1), None, Some(2), None, Some(3)]);
        let result = Solution::lca_deepest_leaves(tree);
        assert_eq!(result.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_1123_5_complex_tree_lca_root() {
        // Tree structure:
        //         1
        //        / \
        //       2   3
        //      / \   \
        //     4   5   6
        //    /         \
        //   7           8
        let tree = build_tree(&[
            Some(1), // root
            Some(2), // left child of 1
            Some(3), // right child of 1
            Some(4), // left child of 2
            Some(5), // right child of 2
            None,    // left child of 3 (null)
            Some(6), // right child of 3
            Some(7), // left child of 4
            None,    // right child of 4 (null)
            None,    // left child of 5 (null)
            None,    // right child of 5 (null)
            None,    // left child of 6 (null)
            Some(8), // right child of 6
        ]);
        let result = Solution::lca_deepest_leaves(tree);
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_1123_6_deepest_lca_not_root() {
        //     1
        //    / \
        //   2   3
        //  / \
        // 4   5
        let tree = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let result = Solution::lca_deepest_leaves(tree);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_1123_7_leetcode_example_1() {
        // Tree structure:
        //         3
        //        / \
        //       5   1
        //      / \ / \
        //     6  2 0 8
        //       / \
        //      7   4
        let tree = build_tree(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let result = Solution::lca_deepest_leaves(tree);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_1123_8_single_node_tree() {
        // Tree structure:
        //   1
        let tree = build_tree(&[Some(1)]);
        let result = Solution::lca_deepest_leaves(tree);
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_1123_9_asymmetric_tree() {
        // Tree structure:
        //       0
        //      /
        //     1
        //    /
        //   3
        //  /
        // 2
        let tree = build_tree(&[Some(0), Some(1), None, Some(3), None, Some(2)]);
        let result = Solution::lca_deepest_leaves(tree);
        assert_eq!(result.unwrap().borrow().val, 2);
    }
}
