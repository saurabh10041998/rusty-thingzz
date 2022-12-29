use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Task {
    enqueue_time: i64,
    process_time: i64,
    id: usize,
}

impl From<&Vec<i32>> for Task {
    fn from(buffer: &Vec<i32>) -> Self {
        Task {
            enqueue_time: buffer[0] as i64,
            process_time: buffer[1] as i64,
            id: usize::MIN, // set proper it future
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.process_time == other.process_time {
            return self.id.partial_cmp(&other.id);
        }
        self.process_time.partial_cmp(&other.process_time)
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.process_time == other.process_time {
            return self.id.cmp(&other.id);
        }
        self.process_time.cmp(&other.process_time)
    }
}

fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    let mut task_lst = vec![];
    for (i, t) in tasks.iter().enumerate() {
        let mut task: Task = t.into();
        task.id = i;
        task_lst.push(task);
    }
    task_lst.sort_by(|task_a, task_b| task_a.enqueue_time.cmp(&task_b.enqueue_time));
    let n = task_lst.len();
    let mut res = vec![];
    let mut pq = BinaryHeap::new();
    let mut i = 0;
    let mut curr_time = task_lst[i].enqueue_time;
    while i < n || !pq.is_empty() {
        while i < n && curr_time >= task_lst[i].enqueue_time {
            pq.push(Reverse(task_lst[i].clone()));
            i += 1;
        }
        let Reverse(curr_task) = pq.pop().unwrap();
        res.push(curr_task.id as i32);
        curr_time += curr_task.process_time;
        if i < n && curr_time < task_lst[i].enqueue_time && pq.is_empty() {
            curr_time = task_lst[i].enqueue_time;
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
