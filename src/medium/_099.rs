use std::cell::{RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
}

pub struct Solution {}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut curr: Option<Rc<RefCell<TreeNode>>> = root.clone();
        let mut x: Option<Rc<RefCell<TreeNode>>> = None;
        let mut y: Option<Rc<RefCell<TreeNode>>> = None;
        let mut pred: Option<Rc<RefCell<TreeNode>>> = None;

        while !(stack.is_empty() && curr.is_none()) {
            while let Some(node) = curr {
                curr = node.borrow_mut().left.clone();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                if let Some(p) = pred {
                    if p.borrow_mut().val > node.borrow_mut().val {
                        y = Some(node.clone());

                        if x.is_none() {
                            x = Some(p);
                        } else {
                            break;
                        }
                    }
                }

                pred = Some(node.clone());
                curr = node.borrow_mut().right.clone();
            }
        }

        let mut x: RefMut<TreeNode> = x.as_ref().unwrap().borrow_mut();
        let mut y: RefMut<TreeNode> = y.as_ref().unwrap().borrow_mut();
        let temp: i32 = x.val;
        x.val = y.val;
        y.val = temp;
    }
}

pub struct SolutionAlt {}

impl SolutionAlt {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut first: Option<Rc<RefCell<TreeNode>>> = None;
        let mut second: Option<Rc<RefCell<TreeNode>>> = None;
        let mut prev: Rc<RefCell<TreeNode>> = TreeNode::new_rc(i32::MIN);

        Self::traverse(
            root.as_ref().unwrap().clone(),
            &mut first,
            &mut second,
            &mut prev,
        );

        let first: Rc<RefCell<TreeNode>> = first.unwrap();
        let second: Rc<RefCell<TreeNode>> = second.unwrap();

        let x: i32 = first.borrow().val;
        first.borrow_mut().val = second.borrow().val;
        second.borrow_mut().val = x;
    }

    fn traverse(
        root: Rc<RefCell<TreeNode>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Rc<RefCell<TreeNode>>,
    ) {
        if let Some(l) = &root.borrow().left {
            Self::traverse(l.clone(), first, second, prev);
        }

        if first.is_none() && prev.borrow().val > root.borrow().val {
            *first = Some(prev.clone());
        }

        if first.is_some() && prev.borrow().val > root.borrow().val {
            *second = Some(root.clone())
        }

        *prev = root.clone();

        if let Some(r) = &root.borrow().right {
            Self::traverse(r.clone(), first, second, prev);
        }
    }
}

impl Clone for TreeNode {
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl TreeNode {
    #[inline]
    pub fn new_rc(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    pub fn build(v: Vec<Option<i32>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(v.try_into().unwrap()))
    }

    pub fn build_from_str(st: &str) -> Rc<RefCell<Self>> {
        let st: String = st.to_string();
        let v: Vec<Option<i32>> = st.split(",").map(|n| n.parse().ok()).collect();
        Self::build(v)
    }

    fn print_tree_h(curr: Rc<RefCell<Self>>, depth: usize, is_left: bool) -> String {
        let mut rv: String = String::new();

        for i in 0..depth {
            if i < depth - 1 {
                rv.push_str("\u{23B8}   ");
            } else {
                rv.push(if is_left { '\u{0371}' } else { '\u{221F}' });
                rv.push_str("\u{2014}\u{2014}\u{2014}");
            }
        }

        rv.push_str(&curr.borrow().val.to_string());
        rv.push('\n');

        if let Some(l) = &curr.borrow().left {
            rv.push_str(&Self::print_tree_h(l.clone(), depth + 1, true));
        }

        if let Some(r) = &curr.borrow().right {
            rv.push_str(&Self::print_tree_h(r.clone(), depth + 1, false));
        }

        rv
    }
}

impl TryFrom<Vec<Option<i32>>> for TreeNode {
    type Error = &'static str;

    fn try_from(vec: Vec<Option<i32>>) -> Result<Self, Self::Error> {
        let root: Rc<RefCell<TreeNode>> = Self::new_rc(vec[0].ok_or("vec is empty")?);
        let mut queue: std::collections::VecDeque<Rc<RefCell<TreeNode>>> =
            std::collections::VecDeque::from([root.clone()]);
        let mut i = 1;

        while let Some(node) = queue.pop_front() {
            if i == vec.len() {
                break;
            }

            if let Some(x) = vec[i] {
                let left: Rc<RefCell<TreeNode>> = Self::new_rc(x);
                queue.push_back(left.clone());
                node.borrow_mut().left = Some(left);
            }
            i += 1;

            if i == vec.len() {
                break;
            }

            if let Some(x) = vec[i] {
                let right: Rc<RefCell<TreeNode>> = Self::new_rc(x);
                queue.push_back(right.clone());
                node.borrow_mut().right = Some(right);
            }
            i += 1;
        }

        let x: TreeNode = root.borrow().clone();

        Ok(x)
    }
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Self::print_tree_h(Rc::new(RefCell::new(self.clone())), 0, true)
        )
    }
}
