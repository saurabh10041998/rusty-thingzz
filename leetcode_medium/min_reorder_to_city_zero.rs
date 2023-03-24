use std::collections::HashSet;
// What are variant of edge in general ??
pub enum Edge {
    Incoming(usize), // incoming from x
    Outgoing(usize), // outgoing to x
}

fn dfs(graph: &Vec<Vec<Edge>>, visited: &mut HashSet<usize>, src: usize) -> i32 {
    let mut ans = 0;
    visited.insert(src);
    for e in graph[src].iter() {
        ans += match e {
            Edge::Incoming(v) => {
                if !visited.contains(v) {
                    dfs(graph, visited, *v) + 0
                } else {
                    0
                }
            }
            Edge::Outgoing(v) => {
                if !visited.contains(v) {
                    // Outgoing edge needs to changed to incomming type
                    // So current edge will be add 1 to ans
                    dfs(graph, visited, *v) + 1
                } else {
                    0
                }
            }
        };
    }
    ans
}

fn min_order(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut graph: Vec<Vec<Edge>> = vec![];
    graph.resize_with(n as usize, || Vec::new());
    for c in connections {
        let (u, v) = (c[0] as usize, c[1] as usize);
        graph[u].push(Edge::Outgoing(v));
        graph[v].push(Edge::Incoming(u));
    }
    let mut visited = HashSet::new();
    dfs(&graph, &mut visited, 0)
}

#[cfg(test)]
pub mod tests {
    use crate::min_order;
    #[test]
    fn run_tc1() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
        assert_eq!(min_order(n, connections), 3);
    }
    #[test]
    fn run_tc2() {
        let n = 5;
        let connections = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];
        assert_eq!(min_order(n, connections), 2);
    }
    #[test]
    fn run_tc3() {
        let n = 3;
        let connections = vec![vec![1, 0], vec![2, 0]];
        assert_eq!(min_order(n, connections), 0);
    }
}

fn main() {
    let n = 6;
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    assert_eq!(min_order(n, connections), 3);
}
