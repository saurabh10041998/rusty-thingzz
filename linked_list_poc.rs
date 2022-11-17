#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn insert(mut head: Option<Box<ListNode>>, new_node: ListNode) -> Option<Box<ListNode>> {
    if head.is_none() {
        head = Some(Box::new(new_node));
        return head;
    }
    let mut cur = &mut head;
    while cur.as_mut().unwrap().next.is_some() {
        cur = &mut cur.as_mut().unwrap().next;
    }
    match cur {
        Some(node) => {
            node.next = Some(Box::new(new_node));
        }
        None => {
            unreachable!();
        }
    }
    head
}

fn to_list(buffer: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in buffer {
        let new_node = ListNode::new(i);
        head = insert(head, new_node);
    }
    head
}

fn traverse_list(head: &Option<Box<ListNode>>) {
    let mut cur = head;
    while let Some(node) = cur.as_ref() {
        print!("{} -> ", node.val);
        cur = &node.next;
    }
    println!("Nil");
}

// reverse the list

fn reverse_lst(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut prev = None;
    while let Some(mut node) = cur.take() {
        let next_node = std::mem::replace(&mut node.next, prev);
        prev = Some(node);
        cur = next_node;
    }
    prev
}

fn main() {
    println!("Hello, world!");
    let buffer = vec![1, 2, 3, 4, 5, 6];
    let mut head = to_list(buffer);
    traverse_list(&head);
    head = reverse_lst(head);
    traverse_list(&head);
}
