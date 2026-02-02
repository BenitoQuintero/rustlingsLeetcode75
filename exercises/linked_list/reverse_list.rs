// 206. Reverse Linked List
//
// Given the head of a singly linked list, reverse the list, and return the reversed list.

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

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
        let ans = reverse_list(head);
        assert_eq!(ans, arr_to_linked_list(&[5, 4, 3, 2, 1]));
    }

    #[test]
    fn case2() {
        let head = arr_to_linked_list(&[1, 2]);
        let ans = reverse_list(head);
        assert_eq!(ans, arr_to_linked_list(&[2, 1]));
    }

    #[test]
    fn case3() {
        let head = arr_to_linked_list(&[]);
        let ans = reverse_list(head);
        assert_eq!(ans, arr_to_linked_list(&[]));
    }
}
