use std::mem;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Utility to construct list from vector
fn to_list(buffer: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for c in buffer.iter().rev() {
        let mut new_node = ListNode::new(*c);
        new_node.next = head.take();
        head = Some(Box::new(new_node));
    }
    head
}

// Solution
fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut even_dummy_head = ListNode::new(-1);
    let mut odd_dummy_head = ListNode::new(-1);
    let mut even_cur = &mut even_dummy_head;
    let mut odd_cur = &mut odd_dummy_head;
    let mut is_even = false;
    while let Some(mut node) = head {
        head = mem::replace(&mut node.next, None);
        if is_even {
            even_cur.next = Some(node);
            even_cur = even_cur.next.as_mut().unwrap();
        } else {
            odd_cur.next = Some(node);
            odd_cur = odd_cur.next.as_mut().unwrap();
        }
        is_even = !is_even;
    }
    odd_cur.next = even_dummy_head.next;
    odd_dummy_head.next
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(odd_even_list(head), to_list(vec![1, 3, 5, 2, 4]));
    }

    #[test]
    fn run_tc2() {
        let head = to_list(vec![2, 1, 3, 5, 6, 4, 7]);
        assert_eq!(odd_even_list(head), to_list(vec![2, 3, 6, 7, 1, 5, 4]));
    }
}

fn main() {
    let head = to_list(vec![1, 2, 3, 4, 5]);
    assert_eq!(odd_even_list(head), to_list(vec![1, 3, 5, 2, 4]));
}
