// 872. Leaf-Similar Trees
//
// Consider all the leaves of a binary tree, from left to right order,
// the values of those leaves form a leaf value sequence.
//
// For example, in the given tree above, the leaf value sequence is (6, 7, 4, 9, 8).
//
// Two binary trees are considered leaf-similar if their leaf value sequence is the same.
//
// Return true if and only if the two given trees with head nodes root1 and root2 are leaf-similar.

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

fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
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
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(9),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let t2 = arr_to_binary_tree(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(7),
            Some(4),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(9),
            Some(8),
        ]);
        let ans = leaf_similar(t1, t2);
        assert!(ans);
    }

    #[test]
    fn case2() {
        let t1 = arr_to_binary_tree(&[Some(1), Some(2), Some(3)]);
        let t2 = arr_to_binary_tree(&[Some(1), Some(3), Some(2)]);
        let ans = leaf_similar(t1, t2);
        assert!(!ans);
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
