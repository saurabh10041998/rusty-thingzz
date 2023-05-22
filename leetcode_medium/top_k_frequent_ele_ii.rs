use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let mut counter = HashMap::new();
    for n in nums {
        counter.entry(n).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut buckets = vec![vec![]; n + 1];
    for (n, c) in counter.into_iter() {
        buckets[c as usize].push(n);
    }
    buckets
        .into_iter()
        .rev()
        .flatten()
        .take(k as usize)
        .collect()
}

#[cfg(test)]
pub mod tests {
    use crate::top_k_frequent;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(top_k_frequent(nums, k), vec![1, 2]);
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
