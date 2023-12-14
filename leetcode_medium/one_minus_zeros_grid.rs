//! Here we are given equation to compute, if rearrage the equation in the question
//! You get,
//! row[i, j] = ones[i] - zeros[i] + ones[j] - zeros[j]
//!           = ones[i] - (rowlen - ones[i]) + ones[j] - (collen - ones[j])
//!           = 2 * ones[i] - rowlen + 2 * ones[j] - collen
//!           = 2 * (ones[i] + ones[j]) - (rowlen + colen)
//!           = 2 * (ones[i] + ones[j]) - gridval         ...where gridval = rowlen + collen
//!
fn one_minus_zeros(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_ones = grid.iter().map(|r| r.iter().sum()).collect::<Vec<i32>>();
    let row_len = grid.len() as i32;
    let col_ones = (0..grid[0].len())
        .map(|j| grid.iter().map(|r| r[j]).sum())
        .collect::<Vec<i32>>();
    let col_len = grid[0].len() as i32;
    let grid_val = row_len + col_len;
    grid.iter_mut().enumerate().for_each(|(i, r)| {
        r.iter_mut()
            .enumerate()
            .for_each(|(j, v)| *v = 2 * (row_ones[i] + col_ones[j]) - grid_val)
    });
    grid
}

#[cfg(test)]
pub mod tests {
    use crate::one_minus_zeros;
    #[test]
    fn run_tc1() {
        let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
        assert_eq!(
            one_minus_zeros(grid),
            vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]
        );
    }
    #[test]
    fn run_tc2() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(one_minus_zeros(grid), vec![vec![5, 5, 5], vec![5, 5, 5]]);
    }
}
fn main() {
    let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
    assert_eq!(one_minus_zeros(grid), vec![vec![5, 5, 5], vec![5, 5, 5]]);
}
