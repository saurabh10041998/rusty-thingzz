#[derive(PartialEq, Eq, Debug)]
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

// ###########[ MAIN SOLUTION STARTS FROM HERE ]###########
fn count(head: Option<&Box<ListNode>>) -> usize {
    let mut curr = head;
    let mut count = 0;
    while let Some(node) = curr {
        count += 1;
        curr = node.next.as_ref();
    }
    count
}

fn partition_lst(
    mut head: Option<Box<ListNode>>,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mid = count(head.as_ref()) / 2;
    // partition list
    let mut top = None;
    for _ in 0..mid {
        let mut node = head.take().unwrap();
        head = std::mem::replace(&mut node.next, top);
        top = Some(node);
    }
    (top, head)
}

fn reverse_lst(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev.take();
        prev = Some(node);
    }
    prev
}

fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let owned_head = head.take();
    let (top, bottom) = partition_lst(owned_head);
    let mut top = reverse_lst(top);
    let mut bottom = reverse_lst(bottom);
    let mut dummy = ListNode::new(-1);
    let mut curr = &mut dummy;
    // MERGE BABY
    loop {
        match (top, bottom) {
            (Some(mut top_node), Some(mut bottom_node)) => {
                top = top_node.next.take();
                bottom = bottom_node.next.take();
                top_node.next = Some(bottom_node);
                curr.next = Some(top_node);
                curr = curr.next.as_mut().unwrap().next.as_mut().unwrap();
            }
            (Some(top_node), None) => {
                curr.next = Some(top_node);
                break;
            }
            (None, Some(bottom_node)) => {
                curr.next = Some(bottom_node);
                break;
            }
            (None, None) => {
                break;
            }
        }
    }
    *head = dummy.next;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut root = build_lst(vec![1, 2, 3, 4]);
        reorder_list(&mut root);
        assert_eq!(root, build_lst(vec![1, 4, 2, 3]));
    }
    #[test]
    fn run_tc2() {
        let mut root = build_lst(vec![1, 2, 3, 4, 5]);
        reorder_list(&mut root);
        assert_eq!(root, build_lst(vec![1, 5, 2, 4, 3]));
    }
}

fn main() {
    let mut root = build_lst(vec![1, 2, 3, 4, 5]);
    reorder_list(&mut root);
    assert_eq!(root, build_lst(vec![1, 5, 2, 4, 3]));
}
