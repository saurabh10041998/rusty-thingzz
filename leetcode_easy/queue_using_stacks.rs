#[derive(Debug)]
pub struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        while let Some(ele) = self.s1.pop() {
            self.s2.push(ele);
        }
        self.s1.push(x);
        while let Some(ele) = self.s2.pop() {
            self.s1.push(ele);
        }
    }

    fn pop(&mut self) -> i32 {
        // SAFETY : Atleast one greater push call than pop call ahead..
        // Garuntees element in s1
        self.s1.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.s1.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.s1.is_empty()
    }
}

// To see the queue contents..
// use cargo test -- --nocapture
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        println!("{:?}", q);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.peek(), 2);
        assert_eq!(q.empty(), false);
    }
}

fn main() {
    let mut q = MyQueue::new();
    q.push(1);
    q.push(2);
    println!("{:?}", q);
    assert_eq!(q.peek(), 1);
    assert_eq!(q.pop(), 1);
    assert_eq!(q.peek(), 2);
    assert_eq!(q.empty(), false);
}
