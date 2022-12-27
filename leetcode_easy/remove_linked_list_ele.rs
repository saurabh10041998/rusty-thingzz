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
        let mut new_node = ListNode::new(*c);
        new_node.next = head.take();
        head = Some(Box::new(new_node));
    }
    head
}

fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut dummy = None;
    let mut tail = &mut dummy;
    while let Some(mut node) = head {
        head = node.next.take();
        if node.val != val {
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
    dummy
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![1,2,6,3,4,5,6];
        let mut head = build_list(buffer);
        head = remove_elements(head, 6);
        assert_eq!(head, build_list(vec![1,2,3,4,5]));
    }
}


fn main() { 
    let buffer = vec![1,2,6,3,4,5,6];
    let mut head = build_list(buffer);
    head = remove_elements(head, 6);
    dbg!(head);
}