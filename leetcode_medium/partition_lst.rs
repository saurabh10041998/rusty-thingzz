#[derive(Debug, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn build_lst(buffer: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for ele in buffer.into_iter().rev() {
        let mut new_node = Box::new(ListNode::new(ele));
        new_node.next = head.take();
        head = Some(new_node);
    }
    head
}

fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut before = ListNode::new(-1);
    let mut after = ListNode::new(-1);
    let mut before_tail = &mut before;
    let mut after_tail = &mut after;
    while let Some(mut node) = head {
        head = node.next.take();
        if node.val < x {
            before_tail.next = Some(node);
            before_tail = before_tail.next.as_mut().unwrap();
        } else {
            after_tail.next = Some(node);
            after_tail = after_tail.next.as_mut().unwrap();
        }
    }
    before_tail.next = after.next.take();
    before.next.take()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn run_tc1() {
        let buffer = vec![1, 4, 3, 2, 5, 2];
        let head = build_lst(buffer);
        let x = 3;
        assert_eq!(partition(head, x), build_lst(vec![1, 2, 2, 4, 3, 5]));
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![2, 1];
        let head = build_lst(buffer);
        let x = 2;
        assert_eq!(partition(head, x), build_lst(vec![1, 2]));
    }
}

fn main() {
    let buffer = vec![2, 1];
    let head = build_lst(buffer);
    let x = 2;
    assert_eq!(partition(head, x), build_lst(vec![1, 2]));
}
