fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    (0..mat.len()).fold(0, |sum, idx| {
        sum + mat[idx][idx] + mat[idx][mat.len() - 1 - idx]
    }) - (mat.len() as i32 & 1) * mat[mat.len() >> 1][mat.len() >> 1]
}

#[cfg(test)]
pub mod tests {
    use crate::diagonal_sum;
    #[test]
    fn run_tc1() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(diagonal_sum(mat), 25);
    }
    #[test]
    fn run_tc2() {
        let mat = vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(diagonal_sum(mat), 8);
    }
    #[test]
    fn run_tc3() {
        let mat = vec![vec![5]];
        assert_eq!(diagonal_sum(mat), 5);
    }
}

fn main() {
    let mat = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
    ];
    assert_eq!(diagonal_sum(mat), 8);
}
