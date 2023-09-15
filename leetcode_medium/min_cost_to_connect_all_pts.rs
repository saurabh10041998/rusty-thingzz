use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::ops::Sub;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    cost: i32,
    idx: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.cost == other.cost {
            return self.idx.partial_cmp(&other.idx);
        }
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cost == other.cost {
            return self.idx.cmp(&other.idx);
        }
        self.cost.cmp(&other.cost)
    }
}

fn abs<T: PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x > y {
        return x - y;
    }
    y - x
}

fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut adj = HashMap::new();
    for i in 0..n {
        let (x1, y1) = (points[i][0], points[i][1]);
        for j in i + 1..n {
            let (x2, y2) = (points[j][0], points[j][1]);
            let cost = abs(x1, x2) + abs(y1, y2);
            adj.entry(i)
                .and_modify(|v: &mut Vec<Node>| v.push(Node { cost, idx: j }))
                .or_insert(vec![Node { cost, idx: j }]);
            adj.entry(j)
                .and_modify(|v: &mut Vec<Node>| v.push(Node { cost, idx: i }))
                .or_insert(vec![Node { cost, idx: i }]);
        }
    }

    // prims algorithm
    let mut res = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse(Node { cost: 0, idx: 0 })); // Initial cost in 0
    let mut visited = HashSet::new();
    while visited.len() < n {
        let Reverse(Node { cost, idx }) = pq.pop().unwrap();
        if visited.contains(&idx) {
            continue;
        }
        res += cost;
        visited.insert(idx);
        match adj.get(&idx) {
            Some(lst) => {
                for n in lst {
                    if !visited.contains(&n.idx) {
                        pq.push(Reverse(Node {
                            cost: n.cost,
                            idx: n.idx,
                        }))
                    }
                }
            }
            None => {}
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::min_cost_connect_points;
    #[test]
    fn run_tc1() {
        let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
        assert_eq!(min_cost_connect_points(points), 20);
    }
    #[test]
    fn run_tc2() {
        let points = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];
        assert_eq!(min_cost_connect_points(points), 18);
    }
}

fn main() {
    let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    assert_eq!(min_cost_connect_points(points), 20);
}
