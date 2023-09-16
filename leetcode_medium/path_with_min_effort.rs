use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Sub;

#[derive(Debug, PartialEq, Eq)]
struct Tuple {
    dist: i32,
    x: usize,
    y: usize,
}

impl Ord for Tuple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.dist == other.dist {
            return (self.x, self.y).cmp(&(other.x, other.y));
        }
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.dist == other.dist {
            return (self.x, self.y).partial_cmp(&(other.x, other.y));
        }
        self.dist.partial_cmp(&other.dist)
    }
}

fn checked_add_signed(x: isize, y: isize) -> Option<usize> {
    if x + y < 0 {
        return None;
    }
    Some((x + y) as usize)
}

fn abs<T: PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x < y {
        return y - x;
    }
    x - y
}

fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let mut pq = BinaryHeap::new();
    let n = heights.len();
    let m = heights[0].len();
    let mut dist = vec![vec![i32::pow(10, 9); m]; n];
    dist[0][0] = 0;
    pq.push(Reverse(Tuple {
        dist: 0,
        x: 0,
        y: 0,
    }));

    let dr = [-1isize, 0, 1, 0];
    let dc = [0isize, -1, 0, 1];

    while let Some(Reverse(Tuple { dist: effort, x, y })) = pq.pop() {
        if x == n - 1 && y == m - 1 {
            return effort;
        }
        for (&dx, &dy) in dr.iter().zip(dc.iter()) {
            match (
                checked_add_signed(x as isize, dx),
                checked_add_signed(y as isize, dy),
            ) {
                (None, _) => {}
                (_, None) => {}
                (Some(new_x), Some(new_y)) => {
                    if new_x < n && new_y < m {
                        // on the same path, we take maximum of effort so far and new effort to make to go to new_x, new_y
                        let new_effort =
                            i32::max(abs(heights[new_x][new_y], heights[x][y]), effort);
                        // if new_effort is better than effort taken on some other path
                        if new_effort < dist[new_x][new_y] {
                            dist[new_x][new_y] = new_effort;
                            pq.push(Reverse(Tuple {
                                dist: new_effort,
                                x: new_x,
                                y: new_y,
                            }));
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use crate::minimum_effort_path;
    #[test]
    fn run_tc1() {
        let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
        assert_eq!(minimum_effort_path(heights), 2);
    }
    #[test]
    fn run_tc2() {
        let heights = vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]];
        assert_eq!(minimum_effort_path(heights), 1);
    }
    #[test]
    fn run_tc3() {
        let heights = vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1],
        ];
        assert_eq!(minimum_effort_path(heights), 0);
    }
}

fn main() {
    let heights = vec![
        vec![1, 2, 1, 1, 1],
        vec![1, 2, 1, 2, 1],
        vec![1, 2, 1, 2, 1],
        vec![1, 2, 1, 2, 1],
        vec![1, 2, 1, 2, 1],
        vec![1, 1, 1, 2, 1],
    ];
    assert_eq!(minimum_effort_path(heights), 0);
}
