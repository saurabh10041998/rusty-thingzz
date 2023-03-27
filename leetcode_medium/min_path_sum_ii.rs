use std::mem::replace;
fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let r = grid.len();
    let c = grid[0].len();
    let mut prev = vec![i32::MAX; c + 1];
    let mut curr = vec![i32::MAX; c + 1];
    prev[c - 1] = 0;
    for i in (0..r).rev() {
        for j in (0..c).rev() {
            curr[j] = grid[i][j] + i32::min(prev[j], curr[j + 1]);
        }
        prev = replace(&mut curr, vec![i32::MAX; c + 1]);
    }
    prev[0]
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
