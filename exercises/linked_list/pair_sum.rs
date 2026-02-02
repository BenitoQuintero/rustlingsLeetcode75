// 2130. Maximum Twin Sum of a Linked List
//
// In a linked list of size n, where n is even, the ith node (0-indexed) of the
// linked list is known as the twin of the (n-1-i)th node, if 0 <= i <= (n / 2) - 1.
// - For example, if n = 4, then node 0 is the twin of node 3, and node 1 is
//   the twin of node 2. These are the only nodes with twins for n = 4.
//
// The twin sum is defined as the sum of a node and its twin.
//
// Given the head of a linked list with even length, return the maximum twin sum
// of the linked list.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
    // TODO: Complete this function.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    use super::*;

    fn arr_to_linked_list(arr: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in arr.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head.take();
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn case1() {
        let head = arr_to_linked_list(&[5, 4, 2, 1]);
        let ans = pair_sum(head);
        assert_eq!(ans, 6);
    }

    #[test]
    fn case2() {
        let head = arr_to_linked_list(&[4, 2, 2, 3]);
        let ans = pair_sum(head);
        assert_eq!(ans, 7);
    }

    #[test]
    fn case3() {
        let head = arr_to_linked_list(&[1, 100000]);
        let ans = pair_sum(head);
        assert_eq!(ans, 100001);
    }
}
