use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    let mut task_lst = vec![];
    for (i, t) in tasks.iter().enumerate() {
        task_lst.push((t[0], (t[1], i)));
    }
    task_lst.sort();
    let mut pq = BinaryHeap::new();
    let n = task_lst.len();
    let mut i = 0;
    let mut curr_time = task_lst[i].0;
    let mut res = vec![];
    while i < n || !pq.is_empty() {
        while i < n && curr_time >= task_lst[i].0 {
            let t = task_lst[i];
            pq.push(Reverse((t.1 .0, (t.1 .1, t.0))));
            i += 1;
        }
        let Reverse(curr_task) = pq.pop().unwrap();
        res.push(curr_task.1 .0 as i32);
        curr_time += curr_task.0;
        if i < n && curr_time < task_lst[i].0 && pq.is_empty() {
            curr_time = task_lst[i].0;
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::get_order;
    #[test]
    fn run_tc1() {
        let tasks = vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]];
        assert_eq!(get_order(tasks), vec![0, 2, 3, 1]);
    }
    #[test]
    fn run_tc2() {
        let tasks = vec![vec![7, 10], vec![7, 12], vec![7, 5], vec![7, 4], vec![7, 2]];
        assert_eq!(get_order(tasks), vec![4, 3, 2, 0, 1]);
    }
}
fn main() {
    let tasks = vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]];
    assert_eq!(get_order(tasks), vec![0, 2, 3, 1]);
}
