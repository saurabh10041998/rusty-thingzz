use std::collections::VecDeque;

// Types of edges specified
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Edge {
    Red(usize),
    Blue(usize),
}

fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
) -> Vec<i32> {
    let mut res = vec![-1; n as usize];
    let mut visited_red = vec![false; n as usize];
    let mut visited_blue = vec![false; n as usize];
    // Base case
    visited_red[0] = true;
    visited_blue[0] = true;
    res[0] = 0;

    let mut adj_lst = vec![];
    adj_lst.resize_with(n as usize, || Vec::new());
    for e in red_edges.iter() {
        let (u, v) = (e[0] as usize, e[1] as usize);
        adj_lst[u].push(Edge::Red(v));
    }
    for e in blue_edges.iter() {
        let (u, v) = (e[0] as usize, e[1] as usize);
        adj_lst[u].push(Edge::Blue(v));
    }

    let mut dist = 1;
    let mut q = VecDeque::new();
    for e in adj_lst[0].iter() {
        match e {
            Edge::Red(v) => {
                if res[*v] == -1 {
                    res[*v] = 1;
                }
            }
            Edge::Blue(v) => {
                if res[*v] == -1 {
                    res[*v] = 1;
                }
            }
        }
        q.push_back(e.clone());
    }
    while !q.is_empty() {
        let len = q.len();
        dist += 1;
        for _ in 0..len {
            let curr = q.pop_front().unwrap();
            match curr {
                Edge::Red(u) => {
                    visited_red[u] = true;
                    for sub_edge in adj_lst[u].iter() {
                        match sub_edge {
                            Edge::Red(_v) => {
                                continue;
                            }
                            Edge::Blue(v) => {
                                if !visited_blue[*v] {
                                    q.push_back(sub_edge.clone());
                                    if res[*v] == -1 {
                                        res[*v] = dist;
                                    }
                                }
                            }
                        }
                    }
                }
                Edge::Blue(u) => {
                    visited_blue[u] = true;
                    for sub_edge in adj_lst[u].iter() {
                        match sub_edge {
                            Edge::Red(v) => {
                                if !visited_red[*v] {
                                    q.push_back(sub_edge.clone());
                                    if res[*v] == -1 {
                                        res[*v] = dist;
                                    }
                                }
                            }
                            Edge::Blue(_v) => {
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::shortest_alternating_paths;
    #[test]
    fn run_tc1() {
        let n = 3;
        let red_edges = vec![vec![0, 1]];
        let blue_edges = vec![vec![2, 1]];
        assert_eq!(
            shortest_alternating_paths(n, red_edges, blue_edges),
            vec![0, 1, -1]
        );
    }
    #[test]
    fn run_tc2() {
        let n = 3;
        let red_edges = vec![vec![0, 1], vec![1, 2]];
        let blue_edges = vec![];
        assert_eq!(
            shortest_alternating_paths(n, red_edges, blue_edges),
            vec![0, 1, -1]
        );
    }

    #[test]
    fn run_tc3() {
        let n = 5;
        let red_edges = vec![vec![2,2],vec![0,1],vec![0,3],vec![0,0],vec![0,4],vec![2,1],vec![2,0],vec![1,4],vec![3,4]];
        let blue_edges = vec![vec![1,3],vec![0,0],vec![0,3],vec![4,2],vec![1,0]];
        assert_eq!(
            shortest_alternating_paths(n, red_edges, blue_edges),
            vec![0, 1, 2, 1, 1]
        );
    }
}
fn main() {
    let n = 3;
    let red_edges = vec![vec![0, 1]];
    let blue_edges = vec![vec![2, 1]];
    assert_eq!(
        shortest_alternating_paths(n, red_edges, blue_edges),
        vec![0, 1, -1]
    );
}
