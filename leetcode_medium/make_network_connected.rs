use std::collections::HashSet;
use std::collections::VecDeque;

fn bfs(src: usize, visited: &mut HashSet<usize>, adj_lst: &Vec<Vec<usize>>) {
    let mut q = VecDeque::new();
    q.push_back(src);
    visited.insert(src);
    while let Some(v) = q.pop_front() {
        for u in adj_lst[v].iter() {
            if !visited.contains(u) {
                q.push_back(*u);
                visited.insert(*u);
            }
        }
    }
}

fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    if connections.len() < (n - 1) as usize {
        return -1;
    }
    let mut adj_lst = vec![];
    adj_lst.resize_with(n as usize, || Vec::new());
    for c in connections.iter() {
        let (u, v) = (c[0] as usize, c[1] as usize);
        adj_lst[u].push(v);
        adj_lst[v].push(u);
    }
    let mut visited = HashSet::new();
    let mut comp = 0;
    for i in 0..n as usize {
        if !visited.contains(&i) {
            comp += 1;
            bfs(i, &mut visited, &adj_lst);
        }
    }
    comp - 1
}

#[cfg(test)]
pub mod tests {
    use crate::make_connected;
    #[test]
    fn run_tc1() {
        let n = 4;
        let connections = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        assert_eq!(make_connected(n, connections), 1);
    }
    #[test]
    fn run_tc2() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]];
        assert_eq!(make_connected(n, connections), 2);
    }
    #[test]
    fn run_tc3() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]];
        assert_eq!(make_connected(n, connections), -1);
    }
}

fn main() {
    let n = 4;
    let connections = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
    assert_eq!(make_connected(n, connections), 1);
}
