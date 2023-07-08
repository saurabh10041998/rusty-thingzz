use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Heap<T> {
    pq: BinaryHeap<T>,
    k: usize
}

impl <T> Heap<T> 
where T: Ord
{
    fn new(k: usize) -> Self {
        Heap {
            pq: BinaryHeap::with_capacity(k),
            k
        }
    }
    fn push(&mut self, ele: T) {
        self.pq.push(ele);
        if self.pq.len() == self.k {
            self.pq.pop().unwrap();
        }
    }
}

fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let mut min_pq:Heap<Reverse<i64>> = Heap::new(k as usize);
    let mut max_pq:Heap<i64> = Heap::new(k as usize);
    for w in weights.windows(2) {
        assert_eq!(w.len(), 2);
        min_pq.push(Reverse((w[0] + w[1]) as i64));
        max_pq.push((w[0] + w[1]) as i64);
    }
    let max = min_pq.pq.iter().map(|Reverse(x)| x).sum::<i64>();
    let min = max_pq.pq.iter().map(|x| x).sum::<i64>();
    max - min
}

#[cfg(test)]
pub mod tests {
    use crate::put_marbles;
    #[test]
    fn run_tc1() {
        let weights = vec![1, 3, 5, 1];
        let k = 2;
        assert_eq!(put_marbles(weights, k), 4);
    }
    #[test]
    fn run_tc2() {
        let weights = vec![1, 3];
        let k = 2;
        assert_eq!(put_marbles(weights, k), 0);
    }
}

fn main() {
    let weights = vec![1, 3];
    let k = 2;
    assert_eq!(put_marbles(weights, k), 0);
}
