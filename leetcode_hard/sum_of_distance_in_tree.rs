use std::collections::HashSet;

pub struct Solver {
    graph: Vec<HashSet<i32>>,
    n: usize,
    dist: Vec<i32>,
    count: Vec<i32>,
}

impl Solver {
    fn new(graph: Vec<HashSet<i32>>, n: usize) -> Self {
        let mut s = Solver {
            graph,
            n,
            dist: Vec::new(),
            count: Vec::new(),
        };
        s.dist.resize(n, 0);
        s.count.resize(n, 0);
        s
    }

    fn post_order(&mut self, root: usize, parent: Option<usize>) {
        let graph_iter = self.graph[root].clone();
        for v in graph_iter {
            match parent {
                Some(u) => {
                    if u == v as usize {
                        continue;
                    }
                }
                None => {}
            };
            self.post_order(v as usize, Some(root));
            self.count[root] += self.count[v as usize];
            self.dist[root] += self.dist[v as usize] + self.count[v as usize];
        }
        self.count[root] += 1;
    }

    fn pre_order(&mut self, root: usize, parent: Option<usize>) {
        let graph_iter = self.graph[root].clone();
        for v in graph_iter {
            match parent {
                Some(u) => {
                    if u == v as usize {
                        continue;
                    }
                }
                None => {}
            };
            self.dist[v as usize] =
                self.dist[root] - self.count[v as usize] + (self.n as i32 - self.count[v as usize]);
            self.pre_order(v as usize, Some(root));
        }
    }

    fn solve(&mut self) -> Vec<i32> {
        self.post_order(0, None);
        self.pre_order(0, None);
        self.dist.clone()
    }
}

fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph = vec![HashSet::new(); n as usize];
    for v in edges.iter() {
        graph[v[0] as usize].insert(v[1]);
        graph[v[1] as usize].insert(v[0]);
    }
    let mut solver = Solver::new(graph, n as usize);
    solver.solve()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]];
        assert_eq!(
            sum_of_distances_in_tree(n, edges),
            vec![8, 12, 6, 10, 10, 10]
        );
    }

    #[test]
    fn run_tc2() {
        let n = 1;
        let edges = vec![];
        assert_eq!(sum_of_distances_in_tree(n, edges), vec![0]);
    }

    #[test]
    fn run_tc3() {
        let n = 2;
        let edges = vec![vec![1, 0]];
        assert_eq!(sum_of_distances_in_tree(n, edges), vec![1, 1]);
    }
}
fn main() {
    let n = 6;
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]];
    assert_eq!(
        sum_of_distances_in_tree(n, edges),
        vec![8, 12, 6, 10, 10, 10]
    );
}
