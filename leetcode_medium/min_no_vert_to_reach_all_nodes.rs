fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut indegree = vec![0; n as usize];
    for e in edges {
        indegree[e[1] as usize] += 1;
    }
    let mut ans = vec![];
    for i in 0..n as usize {
        if indegree[i] == 0 {
            ans.push(i as i32);
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::find_smallest_set_of_vertices;
    #[test]
    fn run_tc1() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]];
        assert_eq!(find_smallest_set_of_vertices(n, edges), vec![0, 3]);
    }
    #[test]
    fn run_tc2() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]];
        assert_eq!(find_smallest_set_of_vertices(n, edges), vec![0, 2, 3]);
    }
}

fn main() {
    let n = 5;
    let edges = vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]];
    assert_eq!(find_smallest_set_of_vertices(n, edges), vec![0, 2, 3]);    
}
