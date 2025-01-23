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
