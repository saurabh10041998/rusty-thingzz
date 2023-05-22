use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    num: i32,
    count: i32,
}

impl From<(i32, i32)> for Pair {
    fn from(buffer: (i32, i32)) -> Self {
        Pair {
            num: buffer.0,
            count: buffer.1,
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.count == other.count {
            return self.num.partial_cmp(&other.num);
        }
        self.count.partial_cmp(&other.count)
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.count == other.count {
            return self.num.cmp(&other.num);
        }
        self.count.cmp(&other.count)
    }
}

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counter = HashMap::new();
    for n in nums {
        counter.entry(n).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut pq: BinaryHeap<Reverse<Pair>> = BinaryHeap::with_capacity(k as usize + 1);
    counter.into_iter().for_each(|p| {
        pq.push(Reverse(p.into()));
        if pq.len() > k as usize {
            pq.pop().unwrap();
        }
    });
    pq.into_iter().map(|Reverse(p)| p.num).collect()
}

#[cfg(test)]
pub mod tests {
    use crate::top_k_frequent;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(top_k_frequent(nums, k), vec![2, 1]);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1];
        let k = 1;
        assert_eq!(top_k_frequent(nums, k), vec![1]);
    }
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    assert_eq!(top_k_frequent(nums, k), vec![2, 1]);
}
