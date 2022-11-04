#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> ListNode {
        ListNode { val, next: None }
    }
}

fn to_list(ele: Vec<i32>) -> Option<Box<ListNode>> {
    let mut cur = None;
    for &value in ele.iter().rev() {
        let mut new_node = ListNode::new(value);
        new_node.next = cur;
        cur = Some(Box::new(new_node));
    }
    cur
}

fn traverse(head: &Option<Box<ListNode>>) {
    let mut cur = head;
    while let Some(node) = cur {
        println!("{:?}", node.val);
        cur = &node.next;
    }
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
    let mut prev = None;
    let mut cur = head;
    while let Some(mut node) = cur {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        cur = next;
    }
    prev
}

fn main() {
    let ele = vec![0, 1, 2, 3];
    let mut head = to_list(ele);
    traverse(&head);
    head = reverse_list(head);
    println!("{:?}", head);
}
