fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut mat = vec![vec![0; n as usize]; n as usize];
    for q in queries {
        let start = (q[0] as usize, q[1] as usize);
        let end = (q[2] as usize, q[3] as usize);
        for i in start.0..=end.0 {
            for j in start.1..=end.0 {
                mat[i][j] += 1;
            }
        }
    }
    mat
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let n = 3;
        let queries = vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]];
        assert_eq!(
            range_add_queries(n, queries),
            vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]]
        );
    }
    #[test]
    fn run_tc2() {
        let n = 2;
        let queries = vec![vec![0, 0, 1, 1]];
        assert_eq!(range_add_queries(n, queries), vec![vec![1, 1], vec![1, 1]]);
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}
fn main() {
    println!("Hello, world!");
}
