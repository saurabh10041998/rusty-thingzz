use std::mem;

#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> ListNode {
        ListNode { val, next: None }
    }
}

fn to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
    let mut cur = None;
    for &value in vector.iter().rev() {
        let mut new_node = ListNode::new(value);
        new_node.next = cur;
        cur = Some(Box::new(new_node));
    }
    cur
}

// ################### Solution start ################################

// get the length of list
fn len(mut head: Option<&Box<ListNode>>) -> usize {
    let mut count = 0;
    while let Some(node) = head.take() {
        count += 1;
        head = node.next.as_ref();
    }
    count
}

// partition list two parts
fn partition_list(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mid = len(head.as_ref()) / 2;
    let mut top = None;
    for _ in 0..mid {
        let mut node = head.take().unwrap();
        head = mem::replace(&mut node.next, top.take());
        top = Some(node);
    }
    (top, head)
}

fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let (mut top, mut bottom) =  partition_list(head);
    let mut max = i32::MIN;
    while let(Some(mut top_node), Some(mut bottom_node)) = (top.take(), bottom.take()) {
        top = top_node.next.take();
        bottom = bottom_node.next.take();
        let sum = top_node.val + bottom_node.val;
        if sum > max {
            max = sum;
        }
    }
    max
}

// ################### Solution end ################################

fn main() {
    //let testcase = vec![5, 4, 2, 1];
    //let testcase = vec![4,2,2,3];
    let testcase = vec![1, 100000];
    let head = to_list(testcase);
    println!("Part 1: {}", pair_sum(head));    
}
