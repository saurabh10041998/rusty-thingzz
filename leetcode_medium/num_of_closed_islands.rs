use std::collections::HashSet;

fn dfs_helper(
    src: (usize, usize),
    bound: (usize, usize),
    grid: &Vec<Vec<i32>>,
    visited: &mut HashSet<(usize, usize)>,
) -> i32 {
    let r = src.0;
    let c = src.1;
    if grid[r][c] == 1 || visited.contains(&(r, c)) {
        return 1;
    }
    visited.insert((r, c));
    let r1 = match r.checked_sub(1) {
        Some(idx) => dfs_helper((idx, c), bound, grid, visited),
        None => 0,
    };
    let r2 = match r.checked_add(1) {
        Some(idx) if idx == bound.0 => 0,
        Some(idx) => dfs_helper((idx, c), bound, grid, visited),
        None => unreachable!(),
    };
    let r3 = match c.checked_sub(1) {
        Some(idx) => dfs_helper((r, idx), bound, grid, visited),
        None => 0,
    };
    let r4 = match c.checked_add(1) {
        Some(idx) if idx == bound.1 => 0,
        Some(idx) => dfs_helper((r, idx), bound, grid, visited),
        None => 0,
    };
    i32::min(i32::min(r1, r2), i32::min(r3, r4))
}

fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut res = 0;
    let mut visited = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 && !visited.contains(&(i, j)) {
                res += dfs_helper((i, j), (n, m), &grid, &mut visited);
            }
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::closed_island;
    #[test]
    fn run_tc1() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];
        assert_eq!(closed_island(grid), 2);
    }
    #[test]
    fn run_tc2() {
        let grid = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
        ];
        assert_eq!(closed_island(grid), 1);
    }
    #[test]
    fn run_tc3() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];
        assert_eq!(closed_island(grid), 2);
    }
}

fn main() {
    let grid = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 1, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
    ];
    assert_eq!(closed_island(grid), 2);
}
