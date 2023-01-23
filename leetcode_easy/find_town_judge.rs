fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut in_degree = vec![0; (n + 1) as usize];
    let mut out_degree = vec![0; (n + 1) as usize];
    for t in trust.iter() {
        let (u, v) = (t[0] as usize, t[1] as usize);
        out_degree[u] += 1;
        in_degree[v] += 1;
    }
    for i in 1..(n + 1) as usize {
        if in_degree[i] == n - 1 && out_degree[i] == 0 {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
pub mod tests {
    use crate::find_judge;
    #[test]
    fn run_tc1() {
        let n = 2;
        let trust = vec![vec![1, 2]];
        assert_eq!(find_judge(n, trust), 2);
    }
    #[test]
    fn run_tc2() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3]];
        assert_eq!(find_judge(n, trust), 3);
    }
    #[test]
    fn run_tc3() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
        assert_eq!(find_judge(n, trust), -1);
    }
}
fn main() {
    let n = 2;
    let trust = vec![vec![1, 2]];
    assert_eq!(find_judge(n, trust), 2);
}
