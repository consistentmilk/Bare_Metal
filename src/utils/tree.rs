//! Utilities for constructing and printing binary trees in ASCII.
//!
//! - `TreeLink`: Alias for an optional, reference-counted, mutable tree node.
//! - `TreeNode`: Node struct with builders and traversal helpers.
//! - `TreeMaker` trait & `tree!` macro for concise tree literals.

use std::cell::{Ref, RefCell, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;

/// Alias for a shared, mutable link to a `TreeNode`.
pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

/// A node in a binary tree holding an integer value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeNode {
    /// Stored integer value.
    pub val: i32,

    /// Left child node (or `None`).
    pub left: TreeLink,

    /// Right child node (or `None`).
    pub right: TreeLink,
}

impl TreeNode {
    /// Create a new `TreeNode` with the given `val` and no children.
    #[inline]
    pub fn new(val: i32) -> Self {
        // Construct node with value and empty child links.
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// Wrap an integer `val` into a `TreeLink`.
    #[inline]
    fn from_rawval(val: i32) -> TreeLink {
        // Create a new node inside Rc<RefCell> for shared ownership.
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    /// Build a binary tree from a level-order list of `Option<i32>`.
    /// `None` entries represent missing nodes.
    pub fn from_vec(elements: Vec<Option<i32>>) -> TreeLink {
        // If no elements, return an empty tree.
        if elements.is_empty() {
            return None;
        }

        // Initialize root using the first element (guaranteed Some).
        let root_val: i32 = elements[0].unwrap();
        let root: Option<Rc<RefCell<TreeNode>>> = TreeNode::from_rawval(root_val);

        // Queue for BFS traversal of tree nodes.
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        // Safe unwrap because root is Some.
        queue.push_back(root.clone().unwrap());

        // Index for next element in `elements` vector.
        let mut i: usize = 1;

        // Process nodes until queue empty or all elements used.
        while let Some(node_rc) = queue.pop_front() {
            // Borrow node mutably to attach children.
            let mut node_ref: RefMut<'_, TreeNode> = node_rc.borrow_mut();

            // Process left child if within bounds.
            if i < elements.len() {
                if let Some(val) = elements[i] {
                    // Create left child node.
                    let left: Option<Rc<RefCell<TreeNode>>> = TreeNode::from_rawval(val);
                    // Attach to current node.
                    node_ref.left = left.clone();
                    // Enqueue for subsequent child attachment.
                    queue.push_back(left.unwrap());
                }
                i += 1;
            }

            // Process right child if within bounds.
            if i < elements.len() {
                if let Some(val) = elements[i] {
                    // Create right child node.
                    let right: Option<Rc<RefCell<TreeNode>>> = TreeNode::from_rawval(val);
                    // Attach to current node.
                    node_ref.right = right.clone();
                    // Enqueue for subsequent child attachment.
                    queue.push_back(right.unwrap());
                }

                i += 1;
            }
        }

        // Return the constructed tree root.
        root
    }

    /// Print an ASCII-art representation of the tree.
    pub fn print_tree_ascii(root: &TreeLink) {
        // Helper: compute height of the tree recursively.
        fn get_height(node: &TreeLink) -> usize {
            // If node is None, height is 0.
            if let Some(rc) = node {
                let n: Ref<'_, TreeNode> = rc.borrow();

                // Compute left and right subtree heights.
                let hl: usize = get_height(&n.left);
                let hr: usize = get_height(&n.right);

                // Height is 1 + max of children.
                1 + std::cmp::max(hl, hr)
            } else {
                0
            }
        }

        // Helper: fill ASCII lines for each level.
        fn fill_lines(
            node: &TreeLink,
            level: usize,
            lines: &mut Vec<String>,
            pos: usize,
            spacing: usize,
        ) {
            // Process only if node exists.
            if let Some(rc) = node {
                let n: Ref<'_, TreeNode> = rc.borrow();

                // Convert node value to string.
                let s: String = n.val.to_string();

                // Calculate start position for the value.
                let start: usize = pos.saturating_sub(s.len() / 2);

                // Place the value in the corresponding line.
                for (i, ch) in s.chars().enumerate() {
                    if start + i < lines[level * 2].len() {
                        lines[level * 2].replace_range(start + i..start + i + 1, &ch.to_string());
                    }
                }

                // Draw branches if next branch line exists.
                if level * 2 + 1 < lines.len() {
                    let branch_line: &mut String = &mut lines[level * 2 + 1];

                    // Left branch '/'.
                    if n.left.is_some() {
                        let lp: usize = pos.saturating_sub(spacing / 2);

                        if lp < branch_line.len() {
                            branch_line.replace_range(lp..lp + 1, "/");
                        }
                    }

                    // Right branch '\\'.
                    if n.right.is_some() {
                        let rp: usize = pos + spacing / 2;
                        if rp < branch_line.len() {
                            branch_line.replace_range(rp..rp + 1, "\\");
                        }
                    }
                }

                // Recurse for children with halved spacing.
                let half: usize = spacing / 2;
                fill_lines(
                    &n.left,
                    level + 1,
                    lines,
                    pos.saturating_sub(spacing / 2),
                    half,
                );
                fill_lines(&n.right, level + 1, lines, pos + spacing / 2, half);
            }
        }

        // Compute tree height.
        let h: usize = get_height(root);
        // Nothing to print for empty tree.
        if h == 0 {
            return;
        }

        // Calculate width and initial spacing.
        let width: usize = (1 << h) * 3 - 1;
        let spacing: usize = (1 << (h - 1)) * 3 - 1;

        // Prepare blank lines for ASCII drawing.
        let mut lines: Vec<String> = vec![" ".repeat(width); h * 2 - 1];

        // Fill in node values and branches.
        fill_lines(root, 0, &mut lines, width / 2, spacing);

        // Print each line trimming trailing spaces.
        for line in lines {
            println!("{}", line.trim_end());
        }
    }
}

/// Trait providing convenient tree constructors.
pub trait TreeMaker {
    /// Create a branch node with value `val` and given children.
    fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        // Wrap children in a new TreeNode.
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    /// Create a leaf node with value `val` (no children).
    fn leaf(val: i32) -> TreeLink {
        // Equivalent to branch with empty children.
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}

impl TreeMaker for TreeLink {}

/// Macro for concise tree literal creation.
#[macro_export]
macro_rules! tree {
    // Single value becomes a leaf.
    ($e:expr) => {
        TreeLink::leaf($e)
    };
    // Value with left and right subtrees.
    ($e:expr, $l:expr, $r:expr) => {
        TreeLink::branch($e, $l, $r)
    };
}
