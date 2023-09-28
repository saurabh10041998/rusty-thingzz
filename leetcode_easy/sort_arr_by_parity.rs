use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[derive(Debug, PartialEq, Eq)]
struct Number(i32);

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 % 2).cmp(&(other.0 % 2))
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0 % 2).partial_cmp(&(other.0 % 2))
    }
}

fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut pq = BinaryHeap::new();
    for n in nums {
        pq.push(Reverse(Number(n)));
    }

    let mut ans = vec![];
    while let Some(Reverse(Number(x))) = pq.pop() {
        ans.push(x);
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::sort_array_by_parity;

    fn check(ans: Vec<i32>) -> bool {
        let n = ans.len();
        if n == 1 {
            return ans[0] % 2 == 0;
        }
        for i in 0..n / 2 {
            if ans[i] % 2 == 1 {
                return false;
            }
        }
        for i in n / 2..n {
            if ans[i] % 2 == 0 {
                return false;
            }
        }
        true
    }

    #[test]
    fn run_tc1() {
        let nums = vec![3, 1, 2, 4];
        let ans = sort_array_by_parity(nums);
        assert!(check(ans));
    }
    #[test]
    fn run_tc2() {
        let nums = vec![0];
        let ans = sort_array_by_parity(nums);
        assert!(check(ans));
    }
}

fn main() {
    let nums = vec![0];
    let ans = sort_array_by_parity(nums);
    print!("{:?}", ans);
}
