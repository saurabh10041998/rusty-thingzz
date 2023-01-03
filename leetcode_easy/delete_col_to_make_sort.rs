fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut count = 0;
    let grid = strs
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let n = grid.len();
    let m = grid[0].len();
    for col in 0..m {
        let mut prev = grid[0][col];
        for row in 1..n {
            if grid[row][col] < prev {
                count += 1;
                break;
            }
            prev = grid[row][col];
        }
    }
    count
}

#[cfg(test)]
pub mod tests {
    use crate::min_deletion_size;
    #[test]
    fn run_tc1() {
        let strs = vec![
            String::from("cba"),
            String::from("daf"),
            String::from("ghi"),
        ];
        assert_eq!(min_deletion_size(strs), 1);
    }
    #[test]
    fn run_tc2() {
        let strs = vec![String::from("a"), String::from("b")];
        assert_eq!(min_deletion_size(strs), 0);
    }
    #[test]
    fn run_tc3() {
        let strs = vec![
            String::from("zyx"),
            String::from("wvu"),
            String::from("tsr"),
        ];
        assert_eq!(min_deletion_size(strs), 3);
    }
}

fn main() {
    let strs = vec![
        String::from("cba"),
        String::from("daf"),
        String::from("ghi"),
    ];
    assert_eq!(min_deletion_size(strs), 1);
}
