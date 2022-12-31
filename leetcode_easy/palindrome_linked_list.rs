use std::mem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

fn build_list(buffer: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &ele in buffer.iter().rev() {
        let mut new_node = Box::new(ListNode::new(ele));
        new_node.next = head.take();
        head = Some(new_node);
    }
    head
}

fn count_nodes(head: &Option<Box<ListNode>>) -> i32 {
    let mut count = 0;
    let mut curr = head;
    while let Some(node) = curr {
        count += 1;
        curr = &node.next;
    }
    count
}

fn partition_list(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut top = None;
    let mid = count_nodes(&head) / 2;
    for _ in 0..mid {
        let mut node = head.take().unwrap();
        head = mem::replace(&mut node.next, top.take());
        top = Some(node);
    }
    (top, head)
}

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let head = head;
    let mut is_odd = false;
    let n = count_nodes(&head);
    if n == 1 {
        return true;
    }
    if n % 2 != 0 {
        is_odd = true;
    }
    let (top, bottom) = partition_list(head); 
    let mut top_node_ref = &top;
    let mut bottom_node_ref = if is_odd {
        &bottom.as_ref().unwrap().next
    }else {
        &bottom
    };
    while let (Some(top_node), Some(bottom_node)) = (top_node_ref, bottom_node_ref) {
        top_node_ref = &top_node.next;
        bottom_node_ref = &bottom_node.next;
        if top_node.val != bottom_node.val {
            return false;
        }
    }
    true
}

#[cfg(test)]
pub mod tests  {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![1,2,2,1];
        let head = build_list(buffer);
        assert_eq!(is_palindrome(head), true);
    }

    #[test]
    fn run_tc2() {
        let buffer = vec![1,2];
        let head = build_list(buffer);
        assert_eq!(is_palindrome(head), false);
    }
}

fn main() {
    let buffer = vec![1,2,2,1];
    let head = build_list(buffer);
    assert_eq!(is_palindrome(head), true);
}