#[derive(Debug, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn reverse_lst(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut tmp = None;
    while let Some(mut head_node) = head {
        head = head_node.next.take();
        head_node.next = tmp.take();
        tmp = Some(head_node);
    }
    return tmp;
}

fn add(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(-1));
    let mut curr = &mut dummy_head;
    let mut carry = 0;
    while l1.is_some() && l2.is_some() {
        let mut l1_node = l1.unwrap();
        let mut l2_node = l2.unwrap();
        let val = l1_node.val + l2_node.val + carry;
        carry = val / 10;
        let val = val % 10;
        curr.next = Some(Box::new(ListNode::new(val)));
        curr = curr.next.as_mut().unwrap();
        l1 = l1_node.next.take();
        l2 = l2_node.next.take();
    }
    while l1.is_some() {
        let mut l1_node = l1.unwrap();
        let val = l1_node.val + carry;
        carry = val / 10;
        let val = val % 10;
        curr.next = Some(Box::new(ListNode::new(val)));
        curr = curr.next.as_mut().unwrap();
        l1 = l1_node.next.take();
    }
    while l2.is_some() {
        let mut l2_node = l2.unwrap();
        let val = l2_node.val + carry;
        carry = val / 10;
        let val = val % 10;
        curr.next = Some(Box::new(ListNode::new(val)));
        curr = curr.next.as_mut().unwrap();
        l2 = l2_node.next.take();
    }

    while carry != 0 {
        curr.next = Some(Box::new(ListNode::new(carry % 10)));
        curr = curr.next.as_mut().unwrap();
        carry /= 10;
    }

    dummy_head.next
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let l1_rev = reverse_lst(l1);
    let l2_rev = reverse_lst(l2);
    let ans = add(l1_rev, l2_rev);
    reverse_lst(ans)
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

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let l1_buffer = vec![7, 2, 4, 3];
        let l2_buffer = vec![5, 6, 4];
        assert_eq!(
            add_two_numbers(build_lst(l1_buffer), build_lst(l2_buffer)),
            build_lst(vec![7, 8, 0, 7])
        );
    }
    #[test]
    fn run_tc2() {
        let l1_buffer = vec![2, 4, 3];
        let l2_buffer = vec![5, 6, 4];
        assert_eq!(
            add_two_numbers(build_lst(l1_buffer), build_lst(l2_buffer)),
            build_lst(vec![8, 0, 7])
        );
    }
    #[test]
    fn run_tc3() {
        let l1_buffer = vec![0];
        let l2_buffer = vec![0];
        assert_eq!(
            add_two_numbers(build_lst(l1_buffer), build_lst(l2_buffer)),
            build_lst(vec![0])
        );
    }
    #[test]
    fn run_tc4() {
        let l1_buffer = vec![5];
        let l2_buffer = vec![5];
        assert_eq!(
            add_two_numbers(build_lst(l1_buffer), build_lst(l2_buffer)),
            build_lst(vec![1, 0])
        );
    }
}

fn main() {
    let l1_buffer = vec![7, 2, 4, 3];
    let l2_buffer = vec![5, 6, 4];
    assert_eq!(
        add_two_numbers(build_lst(l1_buffer), build_lst(l2_buffer)),
        build_lst(vec![7, 8, 0, 7])
    );
}
