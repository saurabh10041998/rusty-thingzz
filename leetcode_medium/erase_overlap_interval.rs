use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Interval {
    start: i32,
    end: i32,
}
impl From<Vec<i32>> for Interval {
    fn from(value: Vec<i32>) -> Self {
        Interval {
            start: value[0],
            end: value[1],
        }
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.end == other.end {
            return self.start.cmp(&other.start);
        }
        self.end.cmp(&other.end)
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.end == other.end {
            return self.start.partial_cmp(&other.start);
        }
        self.end.partial_cmp(&other.end)
    }
}

fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut pq = BinaryHeap::new();
    for i in intervals {
        let interval: Interval = i.into();
        pq.push(Reverse(interval))
    }
    let mut ans = 0;
    let mut ending_time = i32::MIN;
    while let Some(Reverse(interval)) = pq.pop() {
        if interval.start < ending_time {
            ans += 1;
        } else {
            ending_time = interval.end;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::erase_overlap_intervals;
    #[test]
    fn run_tc1() {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        assert_eq!(erase_overlap_intervals(intervals), 1);
    }
    #[test]
    fn run_tc2() {
        let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
        assert_eq!(erase_overlap_intervals(intervals), 2);
    }
    #[test]
    fn run_tc3() {
        let intervals = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(erase_overlap_intervals(intervals), 0);
    }
}

fn main() {
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    assert_eq!(erase_overlap_intervals(intervals), 1);
}
