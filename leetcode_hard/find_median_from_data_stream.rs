use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianFinder {
    h1: BinaryHeap<i32>,
    h2: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            h1: BinaryHeap::new(),
            h2: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.h1.peek().is_some() && self.h1.peek().unwrap() < &num {
            self.h2.push(Reverse(num));
        } else {
            self.h1.push(num);
        }

        if self.h1.len() > self.h2.len() + 1 {
            let n = self.h1.pop().unwrap();
            self.h2.push(Reverse(n));
        }
        if self.h2.len() > self.h1.len() + 1 {
            let n = self.h2.pop().unwrap().0;
            self.h1.push(n);
        }
    }

    fn find_median(&self) -> f64 {
        if self.h1.len() == self.h2.len() {
            return (*self.h1.peek().unwrap() as f64 + self.h2.peek().unwrap().0 as f64) / 2 as f64;
        }
        if self.h1.len() > self.h2.len() {
            return *self.h1.peek().unwrap() as f64;
        }
        self.h2.peek().unwrap().0 as f64
    }
}

// To see the mutable heaps h1 and h2 after each insertion
//  use cargo test -- --nocapture

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        println!("Heaps now: {:?}, {:?}", obj.h1, obj.h2);
        assert_eq!(obj.find_median(), 1.5);
        obj.add_num(3);
        println!("Heaps now: {:?}, {:?}", obj.h1, obj.h2);
        assert_eq!(obj.find_median(), 2.0);
    }
}

fn main() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    println!("Heaps now: {:?}, {:?}", obj.h1, obj.h2);
    assert_eq!(obj.find_median(), 1.5);
    obj.add_num(3);
    println!("Heaps now: {:?}, {:?}", obj.h1, obj.h2);
    assert_eq!(obj.find_median(), 2.0);
}
