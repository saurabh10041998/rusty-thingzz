fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let r = grid.len();
    let c = grid[0].len();
    let mut dp = vec![vec![i32::MAX; c + 1]; r + 1];
    dp[r][c - 1] = 0;
    for i in (0..r).rev() {
        for j in (0..c).rev() {
            dp[i][j] = grid[i][j] + i32::min(dp[i + 1][j], dp[i][j + 1]);
        }
    }
    dp[0][0]
}

#[cfg(test)]
pub mod tests {
    use crate::min_path_sum;
    #[test]
    fn run_tc1() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(min_path_sum(grid), 7);
    }
    #[test]
    fn run_tc2() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(min_path_sum(grid), 12);
    }
}

fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(min_path_sum(grid), 7);
}
