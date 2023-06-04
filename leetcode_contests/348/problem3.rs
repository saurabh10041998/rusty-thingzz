use std::collections::HashSet;

fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
    let mut row = HashSet::new();
    let mut cols = HashSet::new();
    let mut ans: i64 = 0;
    for q in queries.into_iter().rev() {
        if q[0] == 0 {
            if !row.contains(&q[1]) {
                ans += (n as i64 - cols.len() as i64) * q[2] as i64;
                row.insert(q[1]); 
            }
        } else { 
            if !cols.contains(&q[1]) {
                ans += (n as i64 - row.len() as i64) * q[2] as i64;
                cols.insert(q[1]); 
            }
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let n = 3;
        let queries = vec![vec![0,0,1],vec![1,2,2],vec![0,2,3],vec![1,0,4]];
        assert_eq!(matrix_sum_queries(n, queries), 23);
    }
    #[test]
    fn run_tc2() {
        let n = 3;
        let queries = vec![vec![0,0,4],vec![0,1,2],vec![1,0,1],vec![0,2,3],vec![1,2,1]];
        assert_eq!(matrix_sum_queries(n, queries), 17);
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}

fn main() {
    println!("Hello, world!");
}