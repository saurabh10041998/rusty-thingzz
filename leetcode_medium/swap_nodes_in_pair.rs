macro_rules! next_node_as_ref {
    ($a: expr) => {
        $a.as_ref().unwrap().next
    };
}

macro_rules! next_node_as_mut {
    ($a: expr) => {
        $a.as_mut().unwrap().next
    };
}

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

fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut curr_node = &mut head;
    while curr_node.is_some() && next_node_as_ref!(curr_node).is_some() {
        let mut even_node = next_node_as_mut!(curr_node).take();
        let next_odd_node = next_node_as_mut!(even_node).take();
        next_node_as_mut!(curr_node) = next_odd_node;
        next_node_as_mut!(even_node) = curr_node.take();
        curr_node.replace(even_node.unwrap());
        curr_node = &mut next_node_as_mut!(next_node_as_mut!(curr_node));
    }
    head
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let head = build_lst(vec![1, 2, 3, 4]);
        assert_eq!(swap_pairs(head), build_lst(vec![2, 1, 4, 3]));
    }

    #[test]
    fn run_tc2() {
        let head = build_lst(vec![]);
        assert_eq!(swap_pairs(head), build_lst(vec![]));
    }
    #[test]
    fn run_tc3() {
        let head = build_lst(vec![1]);
        assert_eq!(swap_pairs(head), build_lst(vec![1]));
    }
}

fn main() {
    let head = build_lst(vec![1]);
    assert_eq!(swap_pairs(head), build_lst(vec![1]));
}
