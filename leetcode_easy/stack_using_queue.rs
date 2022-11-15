use std::collections::VecDeque;
use std::mem;

pub struct MyStack {
    q: VecDeque<i32>,
    temp: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            q: VecDeque::new(),
            temp: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.temp.push_back(x);
        while let Some(n) = self.q.pop_front() {
            self.temp.push_back(n);
        }
        mem::swap(&mut self.temp, &mut self.q);
    }

    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q.len() == 0
    }
}

#[cfg(test)]
pub mod tests {
    use crate::MyStack;

    #[test]
    fn run_tc1() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        assert_eq!(obj.pop(), 2);
        assert_eq!(obj.empty(), false);
    }
}

fn main() {
    let mut obj = MyStack::new();
    obj.push(1);
    obj.push(2);
    assert_eq!(obj.top(), 2);
    assert_eq!(obj.pop(), 2);
    assert_eq!(obj.empty(), false);
}
