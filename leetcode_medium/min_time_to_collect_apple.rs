use std::collections::HashSet;

fn dfs(
    adj: &Vec<HashSet<usize>>,
    src: usize,
    visited: &mut Vec<bool>,
    has_apple: &Vec<bool>,
) -> i32 {
    if visited[src] {
        return 0;
    }
    visited[src] = true;
    let mut total_amount = 0;
    for &u in adj[src].iter() {
        if !visited[u] {
            total_amount += dfs(adj, u, visited, has_apple);
        }
    }
    match total_amount {
        // if none of child has apples
        0 => {
            if has_apple[src] && src != 0 {
                // if node has apple and its other than source 0
                2
            } else {
                // no apple or node 0 with apple
                0
            }
        }
        // if childs has some apple,
        v => {
            if src == 0 {
                // if its vertex 0, dont add 2 to ans
                v
            } else {
                // other wise add 2, since upperward edge towards
                // parent will cost 2 extra
                v + 2
            }
        }
    }
}

fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
    let mut visited = vec![false; n as usize];
    let mut adj = vec![HashSet::new(); n as usize];
    for e in edges.iter() {
        let (u, v) = (e[0] as usize, e[1] as usize);
        adj[u].insert(v);
        adj[v].insert(u);
    }
    dfs(&adj, 0, &mut visited, &has_apple)
}

#[cfg(test)]
pub mod tests {
    use crate::min_time;
    #[test]
    fn run_tc1() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, true, false, true, true, false];
        assert_eq!(min_time(n, edges, has_apple), 8);
    }
    #[test]
    fn run_tc2() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, true, false, false, true, false];
        assert_eq!(min_time(n, edges, has_apple), 6);
    }
    #[test]
    fn run_tc3() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 3]];
        let has_apple = vec![true, true, true, true];
        assert_eq!(min_time(n, edges, has_apple), 6);
    }
}

fn main() {
    let n = 7;
    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6],
    ];
    let has_apple = vec![false, false, true, false, true, true, false];
    assert_eq!(min_time(n, edges, has_apple), 8);
}
