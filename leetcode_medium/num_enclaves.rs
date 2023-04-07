use std::collections::HashSet;

fn check(loc: (usize, usize), bound: (usize, usize)) -> bool {
    let row_border = loc.0 == 0 || loc.0 == bound.0 - 1;
    let col_border = loc.1 == 0 || loc.1 == bound.1 - 1;
    row_border || col_border
}

fn dfs(
    src: (usize, usize),
    bound: (usize, usize),
    grid: &Vec<Vec<i32>>,
    visited: &mut HashSet<(usize, usize)>,
) -> i32 {
    let r = src.0;
    let c = src.1;
    if grid[r][c] == 0 || visited.contains(&(r, c)) {
        return 0;
    }
    visited.insert((r, c));

    let r1 = match r.checked_sub(1) {
        Some(idx) => dfs((idx, c), bound, grid, visited),
        None => 0,
    };
    let r2 = match r.checked_add(1) {
        Some(idx) if idx == bound.0 => 0,
        Some(idx) => dfs((idx, c), bound, grid, visited),
        None => unreachable!(),
    };
    let r3 = match c.checked_sub(1) {
        Some(idx) => dfs((r, idx), bound, grid, visited),
        None => 0,
    };
    let r4 = match c.checked_add(1) {
        Some(idx) if idx == bound.1 => 0,
        Some(idx) => dfs((r, idx), bound, grid, visited),
        None => unreachable!(),
    };
    1 + r1 + r2 + r3 + r4
}

fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut visited = HashSet::new();
    let mut total = 0;
    let mut border = 0;

    for i in 0..n {
        for j in 0..m {
            total += grid[i][j];
            if grid[i][j] == 1 && !visited.contains(&(i, j)) {
                if check((i, j), (n, m)) {
                    border += dfs((i, j), (n, m), &grid, &mut visited);
                }
            }
        }
    }
    total - border
}

#[cfg(test)]
pub mod tests {
    use crate::num_enclaves;
    #[test]
    fn run_tc1() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(num_enclaves(grid), 3);
    }
    #[test]
    fn run_tc2() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(num_enclaves(grid), 0);
    }
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 0],
    ];
    assert_eq!(num_enclaves(grid), 3);
}
