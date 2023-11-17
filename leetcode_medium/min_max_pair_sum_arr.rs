use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut minpq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut maxpq: BinaryHeap<i32> = BinaryHeap::new();
    let mut n = nums.len();
    for x in nums {
        minpq.push(Reverse(x));
        maxpq.push(x);
    }
    let mut ans = i32::MIN;
    n /= 2;
    while let (Some(p), Some(Reverse(q))) = (maxpq.pop(), minpq.pop()) {
        n -= 1;
        ans = i32::max(ans, p + q);
        if n == 0 {
            break;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::min_pair_sum;
    #[test]
    fn run_tc1() {
        let nums = vec![3, 5, 2, 3];
        assert_eq!(min_pair_sum(nums), 7);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![3, 5, 4, 2, 4, 6];
        assert_eq!(min_pair_sum(nums), 8);
    }
}

fn main() {
    let nums = vec![3, 5, 4, 2, 4, 6];
    assert_eq!(min_pair_sum(nums), 8);
}
