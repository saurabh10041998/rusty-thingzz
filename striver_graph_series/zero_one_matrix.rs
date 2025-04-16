use std::collections::VecDeque;

struct Cell {
    row: usize,
    col: usize,
    dist: i32,
}

fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = mat.len();
    let m = mat[0].len();
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];
    let mut ans = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 0 {
                ans[i][j] = 0;
                visited[i][j] = true;
                q.push_back(Cell {
                    row: i,
                    col: j,
                    dist: 0,
                });
            }
        }
    }

    let delrow = vec![-1, 0, 1, 0];
    let delcol = vec![0, 1, 0, -1];

    while let Some(cell) = q.pop_front() {
        let row = cell.row;
        let col = cell.col;
        let dist = cell.dist;
        ans[row][col] = dist;

        for i in 0..4 {
            let nrow = row as i32 + delrow[i];
            let ncol = col as i32 + delcol[i];
            if nrow >= 0
                && nrow < n as i32
                && ncol >= 0
                && ncol < m as i32
                && !visited[nrow as usize][ncol as usize]
            {
                q.push_back(Cell {
                    row: nrow as usize,
                    col: ncol as usize,
                    dist: dist + 1,
                });
                visited[nrow as usize][ncol as usize] = true;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::update_matrix;
    #[test]
    fn run_tc1() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(
            update_matrix(mat),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn run_tc2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        assert_eq!(
            update_matrix(mat),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }
}

fn main() {
    let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(
        update_matrix(mat),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
    );
}
