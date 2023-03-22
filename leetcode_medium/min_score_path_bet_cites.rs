use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pair {
    first: usize,
    second: u32,
}

impl From<(i32, i32)> for Pair {
    fn from(value: (i32, i32)) -> Self {
        Pair {
            first: value.0 as usize,
            second: value.1 as u32,
        }
    }
}

fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut adj_lst: Vec<Vec<Pair>> = vec![];
    adj_lst.resize_with((n + 1) as usize, || Vec::new());
    for r in roads.iter() {
        adj_lst[r[0] as usize].push((r[1], r[2]).into());
        adj_lst[r[1] as usize].push((r[0], r[2]).into());
    }
    let mut q = VecDeque::new();
    let mut ans = u32::MAX;
    q.push_back(Pair {
        first: 1,
        second: u32::MAX,
    });
    let mut visited = HashSet::new();
    visited.insert(1);
    while let Some(e) = q.pop_front() {
        for v in adj_lst[e.first].iter() {
            if !visited.contains(&v.first) {
                q.push_back(v.clone());
                visited.insert(v.first);
            }
            ans = u32::min(ans, v.second);
        }
    }
    ans as i32
}

#[cfg(test)]
pub mod tests {
    use crate::min_score;
    #[test]
    fn run_tc1() {
        let n = 4;
        let roads = vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]];
        assert_eq!(min_score(n, roads), 5);
    }
    #[test]
    fn run_tc2() {
        let n = 4;
        let roads = vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]];
        assert_eq!(min_score(n, roads), 2);
    }
}

fn main() {
    let n = 4;
    let roads = vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]];
    assert_eq!(min_score(n, roads), 2);
}
