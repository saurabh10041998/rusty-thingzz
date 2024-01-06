use std::cmp::Ordering;
#[derive(PartialEq, Eq)]
struct Job {
    start_time: i32,
    end_time: i32,
    profit: i32,
}

impl From<(i32, i32, i32)> for Job {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            start_time: value.0,
            end_time: value.1,
            profit: value.2,
        }
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.end_time.partial_cmp(&other.end_time)
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.end_time.cmp(&other.end_time)
    }
}

fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut jobs = start_time
        .into_iter()
        .zip(end_time.into_iter())
        .zip(profit.into_iter())
        .map(|((a, b), c)| (a, b, c).into())
        .collect::<Vec<Job>>();
    jobs.sort();
    let n = jobs.len();
    let mut dp = vec![0; n + 1];
    for i in 0..n {
        let Job {
            start_time,
            end_time: _,
            profit,
        } = jobs[i];
        let idx = upper_bound(&jobs, i, start_time);
        dp[i + 1] = i32::max(dp[i], dp[idx] + profit);
    }
    dp[n]
}

fn upper_bound(jobs: &Vec<Job>, end_idx: usize, start_time: i32) -> usize {
    let mut start = 0;
    let mut end = end_idx;
    while start < end {
        let mid = start
            + match end.checked_sub(start) {
                Some(offset) => offset / 2,
                None => break,
            };
        if jobs[mid].end_time <= start_time {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    start
}

#[cfg(test)]
pub mod tests {
    use crate::job_scheduling;
    #[test]
    fn run_tc1() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];
        assert_eq!(job_scheduling(start_time, end_time, profit), 120)
    }
    #[test]
    fn run_tc2() {
        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];
        assert_eq!(job_scheduling(start_time, end_time, profit), 150)
    }
    #[test]
    fn run_tc3() {
        let start_time = vec![1, 1, 1];
        let end_time = vec![2, 3, 4];
        let profit = vec![5, 6, 4];
        assert_eq!(job_scheduling(start_time, end_time, profit), 6)
    }
}

fn main() {
    let start_time = vec![1, 1, 1];
    let end_time = vec![2, 3, 4];
    let profit = vec![5, 6, 4];
    assert_eq!(job_scheduling(start_time, end_time, profit), 150)
}
