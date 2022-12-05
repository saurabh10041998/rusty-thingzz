#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn to_list(buffer: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in buffer.iter().rev() {
        let mut new_node = ListNode::new(*i);
        new_node.next = head.take();
        head = Some(Box::new(new_node));
    }
    head
}

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    slow.clone()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let head = to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(middle_node(head), to_list(vec![3, 4, 5]));
    }

    #[test]
    fn run_tc2() {
        let head = to_list(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(middle_node(head), to_list(vec![4, 5, 6]));
    }
}

fn main() {
    let buffer = vec![1, 2, 3, 4, 5];
    let head = to_list(buffer);
    let middle_node = middle_node(head);
    dbg!(middle_node);
}
