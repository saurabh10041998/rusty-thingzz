use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut arr = nums1
        .into_iter()
        .zip(nums2.into_iter())
        .collect::<Vec<(i32, i32)>>();
    arr.sort_by(|a, b| Reverse(a.1).cmp(&Reverse(b.1)));
    let mut arr = arr.into_iter().collect::<VecDeque<(i32, i32)>>();
    let mut sum = 0;
    let mut res = 0;
    let mut pq = BinaryHeap::with_capacity(k as usize + 1);
    while let Some((x, y)) = arr.pop_front() {
        sum += x as i64;
        pq.push(Reverse(x));
        if pq.len() > k as usize {
            let smallest_from_n1_which_dont_contribute_to_sum = pq.pop().unwrap().0;
            sum -= smallest_from_n1_which_dont_contribute_to_sum as i64;
        }
        if pq.len() == k as usize {
            // calculation time
            res = i64::max(res, sum as i64 * y as i64);
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::max_score;
    #[test]
    fn run_tc1() {
        let nums1 = vec![1, 3, 3, 2];
        let nums2 = vec![2, 1, 3, 4];
        let k = 3;
        assert_eq!(max_score(nums1, nums2, k), 12);
    }
    #[test]
    fn run_tc2() {
        let nums1 = vec![4, 2, 3, 1, 1];
        let nums2 = vec![7, 5, 10, 9, 6];
        let k = 1;
        assert_eq!(max_score(nums1, nums2, k), 30);
    }
}

fn main() {
    let nums1 = vec![4, 2, 3, 1, 1];
    let nums2 = vec![7, 5, 10, 9, 6];
    let k = 1;
    assert_eq!(max_score(nums1, nums2, k), 30);
}
