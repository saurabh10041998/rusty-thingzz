fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for g in grid {
        let cnt = g
            .into_iter()
            .map(|c| if c < 0 { 1 } else { 0 })
            .sum::<i32>();
        ans += cnt;
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::count_negatives;
    #[test]
    fn run_tc1() {
        let grid = vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3],
        ];
        assert_eq!(count_negatives(grid), 8);
    }
    #[test]
    fn run_tc2() {
        let grid = vec![vec![3, 2], vec![1, 0]];
        assert_eq!(count_negatives(grid), 0);
    }
}

fn main() {
    let grid = vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
    ];
    assert_eq!(count_negatives(grid), 8);
}
