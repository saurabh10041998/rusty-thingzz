fn dfs(
    src: usize,
    graph: &Vec<Vec<i32>>,
    visited: &mut Vec<bool>,
    dfs_visited: &mut Vec<bool>,
    check: &mut Vec<bool>,
) -> bool {
    visited[src] = true;
    dfs_visited[src] = true;
    check[src] = false;

    for &nei in graph[src].iter() {
        let nei = nei as usize;
        if visited[nei] == false {
            if dfs(nei, graph, visited, dfs_visited, check) {
                return true;
            }
        } else {
            if dfs_visited[nei] == true {
                return true;
            }
        }
    }

    check[src] = true;
    dfs_visited[src] = false;
    false
}

fn eventual_safe_node(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut dfs_visited = vec![false; n];
    let mut check = vec![false; n];

    for i in 0..n {
        if visited[i] == false {
            dfs(i, &graph, &mut visited, &mut dfs_visited, &mut check);
        }
    }

    let mut safe_nodes = vec![];
    for i in 0..n {
        if check[i] == true {
            safe_nodes.push(i as i32);
        }
    }

    safe_nodes
}

#[cfg(test)]
mod tests {
    use crate::eventual_safe_node;
    #[test]
    fn run_tc1() {
        let graph = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        assert_eq!(eventual_safe_node(graph), vec![2, 4, 5, 6]);
    }
    #[test]
    fn run_tc2() {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        assert_eq!(eventual_safe_node(graph), vec![4]);
    }
}

fn main() {
    let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
    assert_eq!(eventual_safe_node(graph), vec![4]);
}
