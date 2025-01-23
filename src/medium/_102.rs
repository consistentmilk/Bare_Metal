use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::utils::tree::*;

pub struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut vd: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(tr) = root {
            vd.push_back(tr);
        }

        let mut ans: Vec<Vec<i32>> = Vec::new();

        while !vd.is_empty() {
            let mut row: Vec<i32> = Vec::new();

            for _ in 0..vd.len() {
                if let Some(node) = vd.pop_front() {
                    row.push(node.borrow().val);

                    if let Some(left) = node.borrow_mut().left.clone() {
                        vd.push_back(left);
                    }

                    if let Some(right) = node.borrow_mut().right.clone() {
                        vd.push_back(right);
                    }
                }
            }

            ans.push(row);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    //
    //      3
    //     / \
    //    9  20
    //       /\
    //      15 7
    #[test]
    fn test_102_one() {
        let root: Option<Rc<RefCell<TreeNode>>> =
            tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
        let expected: Vec<Vec<i32>> = vec![vec![3], vec![9, 20], vec![15, 7]];

        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn test_102_two() {
        let root: Option<Rc<RefCell<TreeNode>>> = tree!(1);
        let expected: Vec<Vec<i32>> = vec![vec![1]];

        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn test_102_three() {
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(Solution::level_order(root), expected);
    }
}
