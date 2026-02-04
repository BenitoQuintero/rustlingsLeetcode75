// 104. Maximum Depth of Binary Tree
//
// Given the root of a binary tree, return its maximum depth.
//
// A binary tree's maximum depth is the number of nodes along the longest
// path from the root node down to the farthest leaf node.

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

//Definition for a binary tree node.
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

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // TODO: Complete this function.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn case1() {
        let t1 = arr_to_binary_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        let ans = max_depth(t1);
        assert_eq!(ans, 3);
    }

    #[test]
    fn case2() {
        let t1 = arr_to_binary_tree(&[Some(1), None, Some(2)]);
        let ans = max_depth(t1);
        assert_eq!(ans, 2);
    }

    fn arr_to_binary_tree(arr: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.is_empty() || arr[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(arr[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;

        while i < arr.len() {
            let current = queue.pop_front().unwrap();

            // Left child
            if i < arr.len() {
                if let Some(val) = arr[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    current.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }

            // Right child
            if i < arr.len() {
                if let Some(val) = arr[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    current.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }
}
