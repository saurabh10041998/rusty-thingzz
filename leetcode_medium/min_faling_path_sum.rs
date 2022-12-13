fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    if n == 1 && m == 1 {
        return matrix[0][0];
    }
    let mut dp: Vec<Vec<i32>> = vec![];
    dp.resize_with(m, || {
        let mut x = Vec::new();
        x.resize(n, i32::MAX);
        x
    });
    let mut min = i32::MAX;
    for i in 0..n {
        min = i32::min(min, solve(&mut dp, &matrix, 0, i, m, n));
    }
    min
}

fn solve(
    dp: &mut Vec<Vec<i32>>,
    matrix: &Vec<Vec<i32>>,
    row: usize,
    col: usize,
    num_rows: usize,
    num_cols: usize,
) -> i32 {
    if dp[row][col] != i32::MAX {
        return dp[row][col];
    }
    if row == (num_rows - 1) as usize {
        dp[row][col] = matrix[row][col];
        return dp[row][col];
    }
    let left_sum = match col.checked_sub(1) {
        Some(new_col) => solve(dp, matrix, row + 1, new_col, num_rows, num_cols),
        None => i32::MAX,
    };
    let mut right_sum = i32::MAX;
    if col + 1 < num_cols {
        right_sum = solve(dp, matrix, row + 1, col + 1, num_rows, num_cols);
    }

    let straight = solve(dp, matrix, row + 1, col, num_rows, num_cols);

    dp[row][col] = i32::min(left_sum, i32::min(right_sum, straight)) + matrix[row][col];
    dp[row][col]
}

#[cfg(test)]
pub mod tests {
    use crate::min_falling_path_sum;
    #[test]
    fn run_tc1() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(min_falling_path_sum(matrix), 13);
    }

    #[test]
    fn run_tc2() {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        assert_eq!(min_falling_path_sum(matrix), -59);
    }
}

fn main() {
    //println!("Hello, world!");
    let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    assert_eq!(min_falling_path_sum(matrix), 13);
}
