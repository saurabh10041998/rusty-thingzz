use std::collections::HashSet;

fn dfs(
    adj_lst: &Vec<HashSet<i32>>,
    visited: &mut HashSet<i32>,
    source: i32,
    destination: i32,
) -> bool {
    if source == destination {
        return true;
    }
    visited.insert(source);
    for e in adj_lst[source as usize].iter() {
        let result = visited.contains(e);
        if !result {
            if dfs(adj_lst, visited, *e, destination) {
                return true;
            }
        }
    }
    false
}

fn valid_graph(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut adj_lst: Vec<HashSet<i32>> = vec![HashSet::new(); n as usize];
    for e in edges.iter() {
        adj_lst[e[0] as usize].insert(e[1]);
        adj_lst[e[1] as usize].insert(e[0]);
    }
    let mut visited = HashSet::new();
    visited.insert(source);
    if dfs(&adj_lst, &mut visited, source, destination) {
        return true;
    }
    false
}

#[cfg(test)]
pub mod tests {
    use crate::valid_graph;
    #[test]
    fn run_tc1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let (source, destination) = (0, 2);
        assert_eq!(valid_graph(n, edges, source, destination), true);
    }

    #[test]
    fn run_tc2() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![1, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
        let (source, destination) = (0, 5);
        assert_eq!(valid_graph(n, edges, source, destination), false);
    }
}

fn main() {
    //println!("Hello, world!");
    let n = 3;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
    let (source, destination) = (0, 2);
    assert_eq!(valid_graph(n, edges, source, destination), true);
}
