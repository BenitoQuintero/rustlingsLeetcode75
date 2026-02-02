// 328. Odd Even Linked List
//
// Given the head of a singly linked list, group all the nodes with odd indices
// together followed by the nodes with even indices, and return the reordered list.
//
// The first node is considered odd, and the second node is even, and so on.
//
// Note that the relative order inside both the even and odd groups should remain
// as it was in the input.
//
// You must solve the problem in O(1) extra space complexity and O(n) time complexity.

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

fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
        let head = arr_to_linked_list(&[1, 2, 3, 4, 5]);
        let ans = odd_even_list(head);
        assert_eq!(ans, arr_to_linked_list(&[1, 3, 5, 2, 4]));
    }

    #[test]
    fn case2() {
        let head = arr_to_linked_list(&[2, 1, 3, 5, 6, 4, 7]);
        let ans = odd_even_list(head);
        assert_eq!(ans, arr_to_linked_list(&[2, 3, 6, 7, 1, 5, 4]));
    }
}
