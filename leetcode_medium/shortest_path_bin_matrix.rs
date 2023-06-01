use std::collections::HashSet;
use std::collections::VecDeque;

fn checked_add_signed(x: isize, y: isize) -> Option<usize> {
    if x - y < 0 {
        return None;
    }
    Some((x - y) as usize)
}

fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut q = VecDeque::new();
    q.push_back(((0, 0), 1)); // (src, len)
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    while let Some(((x, y), length)) = q.pop_front() {
        if grid[x][y] == 1 {
            continue;
        }

        if x == n - 1 && y == n - 1 {
            return length;
        }

        // Iterator over all neighbors
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                match (
                    checked_add_signed(x as isize, dx),
                    checked_add_signed(y as isize, dy),
                ) {
                    (_, None) => {}
                    (None, _) => {}
                    (Some(new_x), Some(new_y)) => {
                        if new_x < n && new_y < n {
                            if !visited.contains(&(new_x, new_y)) {
                                q.push_back(((new_x, new_y), length + 1));
                                visited.insert((new_x, new_y));
                            }
                        }
                    }
                };
            }
        }
    }
    -1
}

#[cfg(test)]
pub mod tests {
    use crate::shortest_path_binary_matrix;
    #[test]
    fn run_tc1() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(shortest_path_binary_matrix(grid), 2);
    }
    #[test]
    fn run_tc2() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(shortest_path_binary_matrix(grid), 4);
    }
    #[test]
    fn run_tc3() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(shortest_path_binary_matrix(grid), -1);
    }
}

fn main() {
    let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(shortest_path_binary_matrix(grid), -1);
}
