use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    pq: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> KthLargest {
        let mut pq = BinaryHeap::with_capacity(k as usize + 1);
        while let Some(num) = nums.pop() {
            pq.push(Reverse(num));
            if pq.len() > k as usize {
                pq.pop().unwrap();
            }
        }
        KthLargest { pq, k: k as usize }
    }
    fn add(&mut self, val: i32) -> i32 {
        self.pq.push(Reverse(val));
        if self.pq.len() > self.k {
            self.pq.pop().unwrap();
        }
        self.pq.peek().unwrap().0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![4, 5, 8, 2];
        let k = 3;
        let mut obj = KthLargest::new(k, nums);
        assert_eq!(obj.add(3), 4);
        assert_eq!(obj.add(5), 5);
        assert_eq!(obj.add(10), 5);
        assert_eq!(obj.add(9), 8);
        assert_eq!(obj.add(4), 8);
    }
}

fn main() {
    let nums = vec![4, 5, 8, 2];
    let k = 3;
    let mut obj = KthLargest::new(k, nums);
    assert_eq!(obj.add(3), 4);
    assert_eq!(obj.add(5), 5);
    assert_eq!(obj.add(10), 5);
    assert_eq!(obj.add(9), 8);
    assert_eq!(obj.add(4), 8);
}
