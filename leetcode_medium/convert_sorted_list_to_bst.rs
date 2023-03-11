use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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
#[derive(Debug)]
struct List;

impl List {
    fn build_list(buffer: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &ele in buffer.iter().rev() {
            let mut new_node = Box::new(ListNode::new(ele));
            new_node.next = head.take();
            head = Some(new_node);
        }
        head
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn build_tree(buffer: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if buffer.len() == 0 {
        return None;
    }
    let root_rc = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut curr = Rc::clone(&root_rc);
    let mut left = true;
    for ele in buffer.iter().skip(1) {
        let new_node;
        match ele {
            Some(val) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(*val))));
                q.push_back(Rc::clone(new_node.as_ref().unwrap()));
            }
            None => {
                new_node = None;
            }
        }
        if left {
            curr = q.pop_front().unwrap();
            curr.borrow_mut().left = new_node;
            left = false;
        } else {
            curr.borrow_mut().right = new_node;
            left = true;
        }
    }
    Some(root_rc)
}

// ************** Solution **************

fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    build_bst(head.as_ref(), None)
}

fn build_bst(
    head: Option<&Box<ListNode>>,
    tail: Option<&Box<ListNode>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if head == tail {
        return None;
    }
    // Get middle node
    let mut slow = head;
    let mut fast = head;
    while fast != tail {
        let fast_next = fast.and_then(|inner| inner.next.as_ref());
        if fast_next == tail {
            break;
        }
        slow = slow.and_then(|inner| inner.next.as_ref());
        fast = fast_next.and_then(|inner| inner.next.as_ref());
    }
    let root_node = Rc::new(RefCell::new(TreeNode::new(slow.unwrap().val)));
    root_node.borrow_mut().left = build_bst(head, slow);
    root_node.borrow_mut().right = build_bst(slow.and_then(|inner| inner.next.as_ref()), tail);
    Some(root_node)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![-10, -3, 0, 5, 9];
        let head = List::build_list(buffer);
        let root = sorted_list_to_bst(head);
        assert_eq!(
            root,
            build_tree(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)])
        );
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![];
        let head = List::build_list(buffer);
        let root = sorted_list_to_bst(head);
        assert_eq!(root, build_tree(vec![]));
    }
}

fn main() {
    let buffer = vec![-10, -3, 0, 5, 9];
    let head = List::build_list(buffer);
    let root = sorted_list_to_bst(head);
    assert_eq!(
        root,
        build_tree(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)])
    );
}
