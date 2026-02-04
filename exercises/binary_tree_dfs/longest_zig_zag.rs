// 1372. Longest ZigZag Path in a Binary Tree
//
// You are given the root of a binary tree.
//
// A ZigZag path for a binary tree is defined as follow:
// - Choose any node in the binary tree and a direction (right or left).
// - If the current direction is right, move to the right child of the current node;
//   otherwise, move to the left child.
// - Change the direction from right to left or from left to right.
// - Repeat the second and third steps until you can't move in the tree.
//
// Zigzag length is defined as the number of nodes visited - 1. (A single node has a length of 0).
//
// Return the longest ZigZag path contained in that tree.

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

fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
            Some(1),
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            None,
            Some(1),
        ]);
        let ans = longest_zig_zag(t1);
        assert_eq!(ans, 3);
    }

    #[test]
    fn case2() {
        let t1 = arr_to_binary_tree(&[
            Some(1),
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
        ]);
        let ans = longest_zig_zag(t1);
        assert_eq!(ans, 4);
    }

    #[test]
    fn case3() {
        let t1 = arr_to_binary_tree(&[Some(1)]);
        let ans = longest_zig_zag(t1);
        assert_eq!(ans, 0);
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
