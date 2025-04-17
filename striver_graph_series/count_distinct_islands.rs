use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq)]
struct Cell {
    row: usize,
    col: usize,
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.row, self.col).hash(state);
    }
}

fn dfs(
    row: usize,
    col: usize,
    grid: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    shape: &mut Vec<Cell>,
    row0: usize,
    col0: usize,
) {
    let n = grid.len();
    let m = grid[0].len();
    visited[row][col] = true;
    shape.push(Cell {
        row: row - row0,
        col: col - col0,
    });
    let delrow = vec![-1, 0, 1, 0];
    let delcol = vec![0, 1, 0, -1];

    for i in 0..4 {
        let nrow = row as i32 + delrow[i];
        let ncol = col as i32 + delcol[i];

        if nrow >= 0
            && nrow < n as i32
            && ncol >= 0
            && ncol < m as i32
            && visited[nrow as usize][ncol as usize] == false
            && grid[nrow as usize][ncol as usize] == 1
        {
            dfs(
                nrow as usize,
                ncol as usize,
                grid,
                visited,
                shape,
                row0,
                col0,
            );
        }
    }
}

fn count_distinct_islands(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();

    let mut visited = vec![vec![false; m]; n];
    let mut shapes: HashSet<Vec<Cell>> = HashSet::new();

    for i in 0..n {
        for j in 0..m {
            if visited[i][j] == false && grid[i][j] == 1 {
                let mut shape = Vec::new();
                dfs(i, j, &grid, &mut visited, &mut shape, i, j);
                shapes.insert(shape);
            }
        }
    }
    shapes.len() as i32
}

#[cfg(test)]
mod tests {
    use crate::count_distinct_islands;
    #[test]
    fn run_tc1() {
        let grid = vec![
            vec![1, 1, 0, 0, 1, 1],
            vec![1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![1, 1, 0, 0, 1, 0],
        ];
        assert_eq!(count_distinct_islands(grid), 2);
    }
}

fn main() {
    let grid = vec![
        vec![1, 1, 0, 0, 1, 1],
        vec![1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1],
        vec![1, 1, 0, 0, 1, 0],
    ];
    assert_eq!(count_distinct_islands(grid), 2);
}
