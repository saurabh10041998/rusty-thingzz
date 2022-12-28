use std::collections::BinaryHeap;
fn min_stones_sum(piles: Vec<i32>, k: i32) -> i32 {
    let mut k = k;
    let mut pq = BinaryHeap::new();
    for c in piles {
        pq.push(c);
    }
    while k > 0 {
        let n = pq.pop().unwrap();
        pq.push(n - n / 2);
        k -= 1;
    }
    let mut sum = 0;
    while let Some(x) = pq.pop() {
        sum += x;
    }
    sum
}

#[cfg(test)]
pub mod tests {
    use crate::min_stones_sum;
    #[test]
    fn run_tc1() {
        let piles = vec![5,4,9];
        let k = 2;
        assert_eq!(min_stones_sum(piles, k), 12);
    }
    #[test]
    fn run_tc2() {
        let piles = vec![4, 3, 6, 7];
        let k = 3;
        assert_eq!(min_stones_sum(piles, k), 12);
    }
}

fn main() {
    let piles = vec![5,4,9];
    let k = 2;
    assert_eq!(min_stones_sum(piles, k), 12);
}
