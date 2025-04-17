use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    RED,
    GREEN,
    NOCOLOR,
}

fn bfs(src: usize, graph: &Vec<Vec<i32>>, color: &mut Vec<Color>) -> bool {
    let mut q = VecDeque::new();
    color[src] = Color::RED;
    q.push_back(src);

    while let Some(node) = q.pop_front() {
        for &nei in graph[node].iter() {
            if color[nei as usize] == Color::NOCOLOR {
                color[nei as usize] = match color[node] {
                    Color::RED => Color::GREEN,
                    Color::GREEN => Color::RED,
                    Color::NOCOLOR => unreachable!("Pushed in the queue without coloring"),
                };
                q.push_back(nei as usize);
            } else {
                if color[nei as usize] == color[node] {
                    return false;
                }
            }
        }
    }
    true
}

fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let n = graph.len();
    let mut color = vec![Color::NOCOLOR; n];
    for i in 0..n {
        if color[i] == Color::NOCOLOR {
            if bfs(i, &graph, &mut color) == false {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
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
