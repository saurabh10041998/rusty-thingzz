#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
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
    for &ele in buffer.iter().rev() {
        let mut new_node = Box::new(ListNode::new(ele));
        new_node.next = head.take();
        head = Some(new_node);
    }
    head
}

fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut curr = head.as_ref();
    let mut from_head = 0;
    while let Some(node) = curr {
        if len + 1 == k {
            // k th node from first
            from_head = node.val;
        }
        curr = node.next.as_ref();
        len += 1;
    }
    let mut curr = &mut head;
    let mut from_tail = 0;
    let mut i = 0;
    while let Some(node) = curr {
        if i + k == len {
            // k th node from last
            from_tail = node.val;
            node.val = from_head;
        }
        curr = &mut node.next;
        i += 1;
    }
    let mut curr = &mut head;
    let mut i = 0;
    while let Some(node) = curr {
        if i + 1 == k {
            node.val = from_tail;
            return head;
        }
        curr = &mut node.next;
        i += 1;
    }
    head
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let head = build_lst(vec![1, 2, 3, 4, 5]);
        let k = 2;
        assert_eq!(swap_nodes(head, k), build_lst(vec![1, 4, 3, 2, 5]));
    }
    #[test]
    fn run_tc2() {
        let head = build_lst(vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5]);
        let k = 5;
        assert_eq!(
            swap_nodes(head, k),
            build_lst(vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5])
        );
    }
}

fn main() {
    let head = build_lst(vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5]);
    let k = 5;
    assert_eq!(
        swap_nodes(head, k),
        build_lst(vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5])
    );
}
