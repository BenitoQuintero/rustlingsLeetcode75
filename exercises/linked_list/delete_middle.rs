// 2095. Delete the Middle Node of a Linked List
//
// You are given the head of a linked list. Delete the middle node,
// and return the head of the modified linked list.
//
// The middle node of a linked list of size n is the ⌊n / 2⌋th node from the start
// using 0-based indexing, where ⌊x⌋ denotes the largest integer less than or equal to x.
//
// For n = 1, 2, 3, 4, and 5, the middle nodes are 0, 1, 1, 2, and 2, respectively.

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

fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
        let head = arr_to_linked_list(&[1, 3, 4, 7, 1, 2, 6]);
        let ans = delete_middle(head);
        assert_eq!(ans, arr_to_linked_list(&[1, 3, 4, 1, 2, 6]));
    }

    #[test]
    fn case2() {
        let head = arr_to_linked_list(&[1, 2, 3, 4]);
        let ans = delete_middle(head);
        assert_eq!(ans, arr_to_linked_list(&[1, 2, 4]));
    }

    #[test]
    fn case3() {
        let head = arr_to_linked_list(&[2, 1]);
        let ans = delete_middle(head);
        assert_eq!(ans, arr_to_linked_list(&[2]));
    }
}
