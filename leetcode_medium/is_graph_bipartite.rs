use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    NoColor,
}

fn bfs(src: usize, graph: &Vec<Vec<i32>>, colors: &mut Vec<Color>) -> bool {
    let mut q = VecDeque::new();
    q.push_back(src);
    colors[src] = Color::Red; // Start with Some color
    while let Some(curr) = q.pop_front() {
        for &v in graph[curr].iter() {
            if colors[v as usize] == Color::NoColor {
                colors[v as usize] = match colors[curr] {
                    Color::Red => Color::Green,
                    Color::Green => Color::Red,
                    Color::NoColor => {
                        unreachable!()
                    }
                };
                q.push_back(v as usize);
            } else if colors[v as usize] == colors[curr] {
                return false;
            }
        }
    }
    true
}

fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let n = graph.len();
    let mut colors = vec![Color::NoColor; n];
    for i in 0..n {
        if colors[i] == Color::NoColor {
            if bfs(i, &graph, &mut colors) == false {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
pub mod tests {
    use crate::is_bipartite;
    #[test]
    fn run_tc1() {
        let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert_eq!(is_bipartite(graph), false);
    }
    #[test]
    fn run_tc2() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert_eq!(is_bipartite(graph), true);
    }
}

fn main() {
    let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
    assert_eq!(is_bipartite(graph), false);
}
