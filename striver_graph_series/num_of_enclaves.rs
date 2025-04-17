use std::collections::VecDeque;

struct Cell {
    row: usize,
    col: usize,
}

fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut visited = vec![vec![false; m]; n];
    let mut q = VecDeque::new();

    // row boundary
    for i in 0..m {
        // first row
        if visited[0][i] == false && grid[0][i] == 1 {
            q.push_back(Cell { row: 0, col: i });
            visited[0][i] = true;
        }
        // last row
        if visited[n - 1][i] == false && grid[n - 1][i] == 1 {
            q.push_back(Cell { row: n - 1, col: i });
            visited[n - 1][i] = true;
        }
    }

    // col boundary
    for i in 0..n {
        // first col
        if visited[i][0] == false && grid[i][0] == 1 {
            q.push_back(Cell { row: i, col: 0 });
            visited[i][0] = true;
        }
        // last col
        if visited[i][m - 1] == false && grid[i][m - 1] == 1 {
            q.push_back(Cell { row: i, col: m - 1 });
            visited[i][m - 1] = true;
        }
    }

    let delrow = vec![-1, 0, 1, 0];
    let delcol = vec![0, 1, 0, -1];

    while let Some(cell) = q.pop_front() {
        let row = cell.row;
        let col = cell.col;

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
                q.push_back(Cell {
                    row: nrow as usize,
                    col: ncol as usize,
                });
                visited[nrow as usize][ncol as usize] = true;
            }
        }
    }
    let mut cnt = 0;

    for i in 0..n {
        for j in 0..m {
            if visited[i][j] == false && grid[i][j] == 1 {
                cnt += 1;
            }
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
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
