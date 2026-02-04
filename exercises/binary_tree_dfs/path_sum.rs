// 437. Path Sum III
//
// Given the root of a binary tree and an integer targetSum, return the number
// of paths where the sum of the values along the path equals targetSum.
//
// The path does not need to start or end at the root or a leaf, but it must go
// downwards (i.e., traveling only from parent nodes to child nodes).

use std::cell::RefCell;
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

fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
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
        let t1 = arr_to_binary_tree(&[
            Some(10),
            Some(5),
            Some(-3),
            Some(3),
            Some(2),
            None,
            Some(11),
            Some(3),
            Some(-2),
            None,
            Some(1),
        ]);
        let ans = path_sum(t1, 8);
        assert_eq!(ans, 3);
    }

    #[test]
    fn case2() {
        let t1 = arr_to_binary_tree(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            Some(5),
            Some(1),
        ]);
        let ans = path_sum(t1, 22);
        assert_eq!(ans, 3);
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
