#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    RED,
    GREEN,
    NOCOLOR,
}

fn dfs(src: usize, clr: Color, graph: &Vec<Vec<i32>>, color: &mut Vec<Color>) -> bool {
    color[src] = clr;

    for &nei in graph[src].iter() {
        if color[nei as usize] == Color::NOCOLOR {
            match color[src] {
                Color::GREEN => {
                    if dfs(nei as usize, Color::RED, graph, color) == false {
                        return false;
                    }
                }
                Color::RED => {
                    if dfs(nei as usize, Color::GREEN, graph, color) == false {
                        return false;
                    }
                }
                Color::NOCOLOR => unreachable!("Please color first, and then explore"),
            };
        } else {
            if color[nei as usize] == color[src] {
                return false;
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
            if dfs(i, Color::RED, &graph, &mut color) == false {
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
