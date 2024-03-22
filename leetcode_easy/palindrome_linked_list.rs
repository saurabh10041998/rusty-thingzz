#[derive(Debug)]
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

fn count(mut head: Option<&Box<ListNode>>) -> usize {
    let mut cnt = 0;
    while let Some(ref node) = head {
        cnt += 1;
        head = node.next.as_ref();
    }
    cnt
}

fn partition_lst(
    mut head: Option<Box<ListNode>>,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let cnt = count(head.as_ref());
    let mid = cnt / 2;
    let mut top = None;
    for _ in 0..mid {
        let mut node = head.take().unwrap();
        head = std::mem::replace(&mut node.next, top.take());
        top = Some(node);
    }
    (top, head)
}

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let is_odd = count(head.as_ref()) % 2 == 1;
    let (top, bottom) = partition_lst(head);
    let mut top_node_ref = top.as_ref();
    let mut bottom_node_ref = if is_odd {
        bottom.as_ref().unwrap().next.as_ref()
    } else {
        bottom.as_ref()
    };

    while let (Some(top_node), Some(bottom_node)) = (top_node_ref, bottom_node_ref) {
        top_node_ref = top_node.next.as_ref();
        bottom_node_ref = bottom_node.next.as_ref();
        if top_node.val != bottom_node.val {
            return false;
        }
    }
    true
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let head = build_lst(vec![1, 2, 2, 1]);
        assert_eq!(is_palindrome(head), true);
    }
    #[test]
    fn run_tc2() {
        let head = build_lst(vec![1, 2]);
        assert_eq!(is_palindrome(head), false);
    }
}

fn main() {
    let head = build_lst(vec![1, 2, 2, 1]);
    assert_eq!(is_palindrome(head), true);
}
