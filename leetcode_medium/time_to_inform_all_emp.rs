use std::collections::HashMap;
use std::collections::VecDeque;

fn num_of_minutes(_n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let mut adj_lst = HashMap::new();
    for (idx, m) in manager.into_iter().enumerate() {
        if m != -1 {
            adj_lst
                .entry(m as usize)
                .and_modify(|v: &mut Vec<usize>| v.push(idx))
                .or_insert_with(|| vec![idx]);
        }
    }

    // BFS
    let mut q = VecDeque::new();
    let mut ans = 0;
    q.push_back((head_id as usize, 0));
    while let Some((id, time)) = q.pop_front() {
        ans = i32::max(ans, time);
        if let Some(subordinates) = adj_lst.get(&id) {
            for &sub in subordinates {
                q.push_back((sub, time + inform_time[id]));
            }
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::num_of_minutes;
    #[test]
    fn run_tc1() {
        let n = 1;
        let head_id = 0;
        let manager = vec![-1];
        let inform_time = vec![0];
        assert_eq!(num_of_minutes(n, head_id, manager, inform_time), 0);
    }
    #[test]
    fn run_tc2() {
        let n = 6;
        let head_id = 2;
        let manager = vec![2, 2, -1, 2, 2, 2];
        let inform_time = vec![0, 0, 1, 0, 0, 0];
        assert_eq!(num_of_minutes(n, head_id, manager, inform_time), 1);
    }
}

fn main() {
    let n = 6;
    let head_id = 2;
    let manager = vec![2, 2, -1, 2, 2, 2];
    let inform_time = vec![0, 0, 1, 0, 0, 0];
    assert_eq!(num_of_minutes(n, head_id, manager, inform_time), 1);
}
