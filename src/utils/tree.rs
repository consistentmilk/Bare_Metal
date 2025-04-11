use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn from_rawval(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    pub fn from_vec(elements: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if elements.is_empty() {
            return None;
        }

        let root_val: i32 = elements[0].unwrap();
        let root: Option<Rc<RefCell<TreeNode>>> = TreeNode::from_rawval(root_val);
        let mut queue: VecDeque<(Option<Rc<RefCell<TreeNode>>>, i32)> =
            VecDeque::from([(root.clone(), 1)]);

        let mut i: usize = 1;

        while !queue.is_empty() {
            let (node, level) = queue.pop_front().unwrap();

            if i < elements.len() && elements[i].is_some() {
                let left_val: i32 = elements[i].unwrap();
                let left_node: Option<Rc<RefCell<TreeNode>>> = TreeNode::from_rawval(left_val);

                if let Some(n) = node.clone() {
                    n.borrow_mut().left = left_node.clone();
                }

                queue.push_back((left_node, level + 1))
            }

            i += 1;

            if i < elements.len() && elements[i].is_some() {
                let right_val: i32 = elements[i].unwrap();
                let right_node: Option<Rc<RefCell<TreeNode>>> = TreeNode::from_rawval(right_val);

                if let Some(n) = node {
                    n.borrow_mut().right = right_node.clone();
                }

                queue.push_back((right_node, level + 1));
            }

            i += 1;
        }

        root
    }

    pub fn print_tree_ascii(root: &Option<Rc<RefCell<TreeNode>>>) {
        fn get_height(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    1 + std::cmp::max(get_height(&n.left), get_height(&n.right))
                }
            }
        }

        fn fill_lines(
            node: &Option<Rc<RefCell<TreeNode>>>,
            level: usize,
            lines: &mut Vec<String>,
            pos: usize,
            spacing: usize,
        ) {
            if let Some(n) = node {
                let n = n.borrow();
                let val_str = n.val.to_string();
                let start = pos - val_str.len() / 2;

                // Fill node value
                let line = &mut lines[level * 2];
                for (i, c) in val_str.chars().enumerate() {
                    if start + i < line.len() {
                        line.replace_range(start + i..start + i + 1, &c.to_string());
                    }
                }

                // Fill branches if not last level
                if level * 2 + 1 < lines.len() {
                    let branch_line = &mut lines[level * 2 + 1];
                    if n.left.is_some() {
                        let left_pos = pos - spacing / 2;
                        if left_pos > 0 {
                            branch_line.replace_range(left_pos..left_pos + 1, "/");
                        }
                    }
                    if n.right.is_some() {
                        let right_pos = pos + spacing / 2;
                        if right_pos < branch_line.len() {
                            branch_line.replace_range(right_pos..right_pos + 1, "\\");
                        }
                    }
                }

                // Recurse for children with reduced spacing
                let new_spacing = spacing / 2;
                fill_lines(&n.left, level + 1, lines, pos - spacing / 2, new_spacing);
                fill_lines(&n.right, level + 1, lines, pos + spacing / 2, new_spacing);
            }
        }

        let height = get_height(root);
        if height == 0 {
            return;
        }

        // Calculate initial spacing (last level has 3 spaces between nodes)
        let spacing = (1 << (height - 1)) * 3 - 1;
        let width = (1 << height) * 3 - 1;
        let mut lines = vec![vec![' '; width].into_iter().collect::<String>(); height * 2 - 1];

        fill_lines(root, 0, &mut lines, width / 2, spacing / 2);

        // Print the lines
        for line in lines {
            println!("{}", line.trim_end());
        }
    }
}

pub trait TreeMaker {
    fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl TreeMaker for TreeLink {}

#[macro_export]
macro_rules! tree {
    ($e: expr) => {
        TreeLink::leaf($e)
    };

    ($e: expr, $l: expr, $r: expr) => {
        TreeLink::branch($e, $l, $r)
    };
}
