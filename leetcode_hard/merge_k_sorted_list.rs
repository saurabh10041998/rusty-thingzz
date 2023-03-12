use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
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

fn build_list(buffer: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &ele in buffer.iter().rev() {
        let mut new_node = Box::new(ListNode::new(ele));
        new_node.next = head.take();
        head = Some(new_node);
    }
    head
}

// ************** Solution **********************
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut pq: BinaryHeap<Reverse<Box<ListNode>>> = BinaryHeap::new();
    for list in lists {
        if let Some(l) = list {
            pq.push(Reverse(l));
        }
    }
    let mut dummy = ListNode::new(-1);
    let mut last = &mut dummy;
    while let Some(Reverse(mut node)) = pq.pop() {
        match node.next.take() {
            Some(next_node) => {
                pq.push(Reverse(next_node));
            }
            None => {}
        };
        last.next = Some(node);
        last = last.next.as_mut().unwrap();
    }
    dummy.next.take()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let lists = vec![
            build_list(vec![1, 4, 5]),
            build_list(vec![1, 3, 4]),
            build_list(vec![2, 6]),
        ];
        assert_eq!(
            merge_k_lists(lists),
            build_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
    }
    #[test]
    fn run_tc2() {
        let lists = vec![];
        assert_eq!(merge_k_lists(lists), build_list(vec![]));
    }
    #[test]
    fn run_tc3() {
        let lists = vec![build_list(vec![])];
        assert_eq!(merge_k_lists(lists), build_list(vec![]));
    }
}

fn main() {
    let lists = vec![
        build_list(vec![1, 4, 5]),
        build_list(vec![1, 3, 4]),
        build_list(vec![2, 6]),
    ];
    assert_eq!(
        merge_k_lists(lists),
        build_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
    );
}
