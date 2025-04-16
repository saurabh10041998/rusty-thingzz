fn dfs(row: usize, col: usize, board: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
    visited[row][col] = true;
    let n = board.len();
    let m = board[0].len();
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
            && board[nrow as usize][ncol as usize] == 'O'
        {
            dfs(nrow as usize, ncol as usize, board, visited);
        }
    }
}

fn solve(board: &mut Vec<Vec<char>>) {
    let n = board.len();
    let m = board[0].len();

    let mut visited = vec![vec![false; m]; n];

    // row boundary dfs
    for i in 0..m {
        // first row
        if visited[0][i] == false && board[0][i] == 'O' {
            dfs(0, i, board, &mut visited);
        }
        // last row
        if visited[n - 1][i] == false && board[n - 1][i] == 'O' {
            dfs(n - 1, i, board, &mut visited);
        }
    }

    // col boundary dfs
    for i in 0..n {
        // first col
        if visited[i][0] == false && board[i][0] == 'O' {
            dfs(i, 0, board, &mut visited);
        }
        // last col
        if visited[i][m - 1] == false && board[i][m - 1] == 'O' {
            dfs(i, m - 1, board, &mut visited);
        }
    }

    for i in 0..n {
        for j in 0..m {
            if visited[i][j] == false && board[i][j] == 'O' {
                board[i][j] = 'X';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;
    #[test]
    fn run_tc1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        solve(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );
    }

    #[test]
    fn run_tc2() {
        let mut board = vec![vec!['X']];
        solve(&mut board);
        assert_eq!(board, vec![vec!['X']]);
    }
}

fn main() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    solve(&mut board);
    assert_eq!(
        board,
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ]
    );
}
