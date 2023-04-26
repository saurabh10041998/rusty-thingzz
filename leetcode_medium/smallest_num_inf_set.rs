use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
pub struct SmallestInfiniteSet {
    pq: BinaryHeap<Reverse<i32>>,
    set: HashSet<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        let pq = (1..=1000)
            .map(|x| Reverse(x))
            .collect::<BinaryHeap<Reverse<i32>>>();
        let set = (1..=1000).collect::<HashSet<i32>>();
        SmallestInfiniteSet { pq, set }
    }

    fn pop_smallest(&mut self) -> i32 {
        match self.pq.pop() {
            Some(Reverse(val)) => {
                self.set.remove(&val);
                val
            }
            None => 0,
        }
    }

    fn add_back(&mut self, num: i32) {
        if !self.set.contains(&num) {
            self.pq.push(Reverse(num));
            self.set.insert(num);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut smallest_infinite_set = SmallestInfiniteSet::new();
        smallest_infinite_set.add_back(2);
        assert_eq!(smallest_infinite_set.pop_smallest(), 1);
        assert_eq!(smallest_infinite_set.pop_smallest(), 2);
        assert_eq!(smallest_infinite_set.pop_smallest(), 3);
        smallest_infinite_set.add_back(1);
        assert_eq!(smallest_infinite_set.pop_smallest(), 1);
        assert_eq!(smallest_infinite_set.pop_smallest(), 4);
        assert_eq!(smallest_infinite_set.pop_smallest(), 5);
    }
}

fn main() {
    let mut smallest_infinite_set = SmallestInfiniteSet::new();
    smallest_infinite_set.add_back(2);
    assert_eq!(smallest_infinite_set.pop_smallest(), 1);
    assert_eq!(smallest_infinite_set.pop_smallest(), 2);
    assert_eq!(smallest_infinite_set.pop_smallest(), 3);
    smallest_infinite_set.add_back(1);
    assert_eq!(smallest_infinite_set.pop_smallest(), 1);
    assert_eq!(smallest_infinite_set.pop_smallest(), 4);
    assert_eq!(smallest_infinite_set.pop_smallest(), 5);
}
