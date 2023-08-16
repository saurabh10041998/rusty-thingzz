use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    first: i32,
    second: usize,
}

impl Pair {
    fn new(first: i32, second: usize) -> Self {
        Pair { first, second }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.first == other.first {
            return self.second.partial_cmp(&other.second);
        }
        self.first.partial_cmp(&other.first)
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.first == other.first {
            return self.second.cmp(&other.second);
        }
        self.first.cmp(&other.first)
    }
}

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut pq = BinaryHeap::new();
    let n = nums.len();
    let mut ans = vec![0; n - k as usize + 1];
    for (idx, &ele) in nums.iter().take(k as usize).enumerate() {
        pq.push(Pair::new(ele, idx));
    }
    ans[0] = pq.peek().unwrap().first;
    for i in k as usize..n {
        while !pq.is_empty() && pq.peek().unwrap().second <= i - k as usize {
            // Remove the elements not in window
            pq.pop().unwrap();
        }
        pq.push(Pair::new(nums[i], i));
        ans[i - k as usize + 1] = pq.peek().unwrap().first;
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::max_sliding_window;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        assert_eq!(max_sliding_window(nums, k), vec![3, 3, 5, 5, 6, 7]);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1];
        let k = 1;
        assert_eq!(max_sliding_window(nums, k), vec![1]);
    }
}

fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    assert_eq!(max_sliding_window(nums, k), vec![3, 3, 5, 5, 6, 7]);
}
