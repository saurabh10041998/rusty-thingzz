use std::collections::{HashSet, VecDeque};
struct Graph<'a> {
    grid: &'a Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
    visited: HashSet<(usize, usize)>,
}

trait Traversal {
    fn dfs(&mut self, src: (usize, usize));
    fn bfs(&mut self) -> i32;
}

impl<'a> Graph<'a> {
    fn new(grid: &'a Vec<Vec<i32>>) -> Self {
        let dim = grid.len();
        Graph {
            grid,
            rows: dim,
            cols: dim,
            visited: HashSet::new(),
        }
    }
}

impl<'a> Traversal for Graph<'a> {
    fn dfs(&mut self, src: (usize, usize)) {
        let (x, y) = (src.0, src.1);
        if self.visited.contains(&src) || self.grid[x][y] == 0 {
            return;
        }
        self.visited.insert(src);
        let direct = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for &(dr, dc) in direct.iter() {
            match (
                x.checked_add_signed(dr as isize),
                y.checked_add_signed(dc as isize),
            ) {
                (None, _) => {}
                (_, None) => {}
                (Some(new_x), Some(new_y)) => {
                    if new_x < self.rows && new_y < self.cols {
                        if !self.visited.contains(&(new_x, new_y)) {
                            self.dfs((new_x, new_y));
                        }
                    }
                }
            }
        }
    }

    fn bfs(&mut self) -> i32 {
        let mut q = VecDeque::new();
        let mut res = 0;
        for &(x, y) in self.visited.iter() {
            q.push_back((x, y));
        }
        let direct = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                let (x, y) = q.pop_front().unwrap();
                for &(dr, dc) in direct.iter() {
                    match (
                        x.checked_add_signed(dr as isize),
                        y.checked_add_signed(dc as isize),
                    ) {
                        (None, _) => {}
                        (_, None) => {}
                        (Some(new_x), Some(new_y)) if self.visited.contains(&(new_x, new_y)) => {}
                        (Some(new_x), Some(new_y)) => {
                            if new_x < self.rows && new_y < self.cols {
                                if self.grid[new_x][new_y] == 1 {
                                    return res;
                                } else {
                                    // water
                                    q.push_back((new_x, new_y));
                                    self.visited.insert((new_x, new_y));
                                }
                            }
                        }
                    }
                }
            }
            res += 1;
        }
        unreachable!()
    }
}

fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
    let mut graph = Graph::new(&grid);
    let n = grid.len();
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                graph.dfs((i, j));
                return graph.bfs();
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use crate::shortest_bridge;
    #[test]
    fn run_tc1() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(shortest_bridge(grid), 1);
    }
    #[test]
    fn run_tc2() {
        let grid = vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]];
        assert_eq!(shortest_bridge(grid), 2);
    }
    #[test]
    fn run_tc3() {
        let grid = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(shortest_bridge(grid), 1);
    }
}

fn main() {
    let grid = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1],
    ];
    assert_eq!(shortest_bridge(grid), 1);
}
