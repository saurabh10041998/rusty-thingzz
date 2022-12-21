use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Solver {
    n: i32,
    adj_lst: Vec<HashSet<i32>>,
    colors: Vec<i32>,
}

impl Solver {
    fn new(adj_lst: Vec<HashSet<i32>>, n: i32) -> Self {
        let mut solver = Solver {
            n,
            adj_lst,
            colors: Vec::new(),
        };
        solver.colors.resize(n as usize, -1);
        solver
    }

    fn bipartite_until(&mut self) -> bool {
        for i in 0..self.n {
            if self.colors[i as usize] == -1 {
                if self.bipartite(i) == false {
                    return false;
                }
            }
        }
        true
    }

    fn bipartite(&mut self, src: i32) -> bool {
        self.colors[src as usize] = 1;
        let mut q = VecDeque::new();
        q.push_back(src);
        while let Some(curr) = q.pop_front() {
            if self.adj_lst[curr as usize].contains(&curr) {
                return false; // self loop
            }
            for c in self.adj_lst[curr as usize].iter() {
                if self.colors[*c as usize] == -1 {
                    self.colors[*c as usize] = 1 - self.colors[curr as usize];
                    q.push_back(*c);
                } else if self.colors[*c as usize] == self.colors[curr as usize] {
                    return false;
                }
            }
        }
        true
    }
}

fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
    let mut graph = vec![HashSet::new(); n as usize];
    for v in dislikes.iter() {
        let (u, v) = (v[0] - 1, v[1] - 1);
        graph[u as usize].insert(v);
        graph[v as usize].insert(u);
    }
    let mut solver = Solver::new(graph, n);

    solver.bipartite_until()
}

#[cfg(test)]
pub mod tests {
    use crate::possible_bipartition;
    #[test]
    fn run_tc1() {
        let n = 4;
        let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
        assert_eq!(possible_bipartition(n, dislikes), true);
    }
    #[test]
    fn run_tc2() {
        let n = 3;
        let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        assert_eq!(possible_bipartition(n, dislikes), false);
    }

    #[test]
    fn run_tc3() {
        let n = 5;
        let dislikes = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
        assert_eq!(possible_bipartition(n, dislikes), false);
    }
}

fn main() {
    let n = 4;
    let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
    assert_eq!(possible_bipartition(n, dislikes), true);
}
