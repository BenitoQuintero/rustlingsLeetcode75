// 236. Lowest Common Ancestor of a Binary Tree
//
// Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
//
// According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between
// two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow
// a node to be a descendant of itself).”

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    // TODO: Complete this function.
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn case1() {
        // Tree: [3,5,1,6,2,0,8,null,null,7,4]
        let root = arr_to_binary_tree(&[
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

        let p = find_node(root.clone(), 5).unwrap();
        let q = find_node(root.clone(), 1).unwrap();

        let ans = lowest_common_ancestor(root, Some(p), Some(q)).unwrap();
        assert_eq!(ans.borrow().val, 3);
    }

    #[test]
    fn case2() {
        // LCA where one node is ancestor of the other
        let root = arr_to_binary_tree(&[
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

        let p = find_node(root.clone(), 5).unwrap();
        let q = find_node(root.clone(), 4).unwrap();

        let ans = lowest_common_ancestor(root, Some(p), Some(q)).unwrap();
        assert_eq!(ans.borrow().val, 5);
    }

    #[test]
    fn case3() {
        // LCA where one node is ancestor of the other
        let root = arr_to_binary_tree(&[Some(1), Some(2)]);

        let p = find_node(root.clone(), 1).unwrap();
        let q = find_node(root.clone(), 2).unwrap();

        let ans = lowest_common_ancestor(root, Some(p), Some(q)).unwrap();
        assert_eq!(ans.borrow().val, 1);
    }

    #[test]
    fn case4() {
        let root = arr_to_binary_tree(&[Some(1)]);
        let p = root.clone();
        let q = root.clone();

        let ans = lowest_common_ancestor(root, p, q).unwrap();
        assert_eq!(ans.borrow().val, 1);
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

            if i < arr.len() {
                if let Some(val) = arr[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    current.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }

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

    fn find_node(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        if root.borrow().val == val {
            return Some(root);
        }
        find_node(root.borrow().left.clone(), val)
            .or_else(|| find_node(root.borrow().right.clone(), val))
    }
}
