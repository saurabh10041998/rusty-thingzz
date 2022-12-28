#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
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
    for c in buffer.iter().rev() { 
        let mut new_node = Box::new(ListNode::new(*c));
        new_node.next = head.take();
        head = Some(new_node);
    }
    head
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev.take();
        prev = Some(node);
    }
    prev
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![1,2,3,4,5];
        let head = build_list(buffer);
        assert_eq!(reverse_list(head), build_list(vec![5,4,3,2,1]));
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![1,2];
        let head = build_list(buffer);
        assert_eq!(reverse_list(head), build_list(vec![2,1]));
    }
    #[test]
    fn run_tc3() {
        let buffer = vec![];
        let head = build_list(buffer);
        assert_eq!(reverse_list(head), build_list(vec![]));
    }
}


fn main() {
    let buffer = vec![1,2,3,4,5];
    let head = build_list(buffer);
    assert_eq!(reverse_list(head), build_list(vec![5,4,3,2,1]));
}