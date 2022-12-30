fn dfs(graph: &Vec<Vec<i32>>, paths: &mut Vec<Vec<i32>>, x: i32, path: &mut Vec<i32>) {
    let dest = graph.len() - 1;
    if x as usize == dest {
        paths.push(path.clone());
        path.pop().unwrap();
        return;
    }
    for v in graph[x as usize].iter() {
        path.push(*v);
        dfs(graph, paths, *v, path);
    }
    path.pop().unwrap();
}

fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut paths = vec![];
    let mut path = vec![];
    path.push(0);
    dfs(&graph, &mut paths, 0, &mut path);
    paths
}

#[cfg(test)]
pub mod tests {
    use crate::all_paths_source_target;
    #[test]
    fn run_tc1() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        assert_eq!(
            all_paths_source_target(graph),
            vec![vec![0, 1, 3], vec![0, 2, 3]]
        );
    }

    #[test]
    fn run_tc2() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        assert_eq!(
            all_paths_source_target(graph),
            vec![
                vec![0, 4],
                vec![0, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 4]
            ]
        );
    }
}

fn main() {
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    assert_eq!(
        all_paths_source_target(graph),
        vec![vec![0, 1, 3], vec![0, 2, 3]]
    );
}
