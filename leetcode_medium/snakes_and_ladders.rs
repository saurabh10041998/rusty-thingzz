use std::collections::VecDeque;

fn snake_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let n = board.len();
    let mut cells = vec![(0, 0); usize::pow(n, 2) + 1];
    let mut cols = (0..n).collect::<Vec<usize>>();
    let mut labels = 1;
    // snake ladder board simulation
    for row in (0..n).rev() {
        for &c in cols.iter() {
            cells[labels] = (row, c);
            labels += 1;
        }
        cols.reverse();
    }
    let mut q = VecDeque::new();
    q.push_back(1); // current cell
    let mut dist = vec![-1; usize::pow(n, 2) + 1];
    dist[1] = 0;
    while let Some(curr) = q.pop_front() {
        for next in curr + 1..=i32::min(curr + 6, i32::pow(n as i32, 2)) {
            let (row, col) = cells[next as usize];
            let destination = if board[row][col] != -1 {
                board[row][col]
            } else {
                next
            };
            if dist[destination as usize] == -1 {
                dist[destination as usize] = dist[curr as usize] + 1;
                q.push_back(destination);
            }
        }
    }
    dist[usize::pow(n, 2)]
}

#[cfg(test)]
pub mod tests {
    use crate::snake_and_ladders;
    #[test]
    fn run_tc1() {
        let board = vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ];
        assert_eq!(snake_and_ladders(board), 4);
    }
    #[test]
    fn run_tc2() {
        let board = vec![vec![-1, -1], vec![-1, 3]];
        assert_eq!(snake_and_ladders(board), 1);
    }
}

fn main() {
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    assert_eq!(snake_and_ladders(board), 4);
}
