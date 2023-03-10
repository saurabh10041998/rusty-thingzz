use rand;
use rand::rngs::ThreadRng;
use rand::Rng;

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

// *********** Solution *****************
struct Solution {
    buffer: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut buffer = vec![];
        let mut curr = &head;
        while let Some(ele) = curr {
            buffer.push(ele.val);
            curr = &ele.next;
        }
        Solution {
            buffer,
            rng: rand::thread_rng(),
        }
    }
    fn get_random(&mut self) -> i32 {
        let idx = self.rng.gen_range(0..self.buffer.len());
        self.buffer[idx]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![1, 2, 3, 4];
        let head = List::build_list(buffer);
        let mut solution = Solution::new(head);
        println!("{}", solution.get_random());
        println!("{}", solution.get_random());
        println!("{}", solution.get_random());
        println!("{}", solution.get_random());
        assert!(true);
    }
}

fn main() {
    let buffer = vec![1, 2, 3, 4];
    let head = List::build_list(buffer);
    let mut solution = Solution::new(head);
    println!("{}", solution.get_random());
    println!("{}", solution.get_random());
    println!("{}", solution.get_random());
    println!("{}", solution.get_random());
    assert!(true);
}
