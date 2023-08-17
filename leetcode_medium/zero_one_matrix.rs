use std::collections::VecDeque;

fn checked_add_signed(x: isize, y: isize) -> Option<usize> {
    if x - y < 0 {
        return None;
    }
    Some((x - y) as usize)
}

fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let max_val = m * n;
    let mut result = mat.clone();
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 0 {
                q.push_back((i, j));
            } else {
                result[i][j] = max_val as i32;
            }
        }
    }
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    while let Some((row, col)) = q.pop_front() {
        for &(dr, dc) in directions.iter() {
            match (
                checked_add_signed(row as isize, dr),
                checked_add_signed(col as isize, dc),
            ) {
                (_, None) => {}
                (None, _) => {}
                (Some(new_r), Some(new_c)) => {
                    if new_r < m && new_c < n && result[new_r][new_c] > result[row][col] + 1 {
                        q.push_back((new_r, new_c));
                        result[new_r][new_c] = result[row][col] + 1;
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
pub mod tests {
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
    let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    assert_eq!(
        update_matrix(mat),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
    );
}
