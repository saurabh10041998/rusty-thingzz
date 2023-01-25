use std::collections::HashSet;

fn dfs(graph: &Vec<HashSet<usize>>, src: usize, visited: &mut HashSet<usize>, dist: &mut Vec<i32>) {
    if visited.contains(&src) {
        return;
    }
    visited.insert(src);
    for v in graph[src].iter() {
        if !visited.contains(v) {
            dist[*v] = dist[src] + 1;
            dfs(graph, *v, visited, dist);
        }
    }
}

fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
    let n = edges.len();
    let mut graph = vec![HashSet::new(); n];
    for (i, &e) in edges.iter().enumerate() {
        if e != -1 {
            graph[i].insert(e as usize);
        }
    }
    let mut dist_from_node1 = vec![i32::MAX; n];
    let mut dist_from_node2 = vec![i32::MAX; n];
    let mut visited_from_node1 = HashSet::new();
    let mut visited_from_node2 = HashSet::new();

    let node1 = node1 as usize;
    let node2 = node2 as usize;
    // Run dfs from node1
    dist_from_node1[node1] = 0;
    dfs(&graph, node1, &mut visited_from_node1, &mut dist_from_node1);

    // Run dfs from node2
    dist_from_node2[node2] = 0;
    dfs(&graph, node2, &mut visited_from_node2, &mut dist_from_node2);

    let mut ans = i32::MAX;
    let mut curr_node = None;
    for i in 0..n {
        if i32::max(dist_from_node1[i], dist_from_node2[i]) < ans {
            ans = i32::max(dist_from_node1[i], dist_from_node2[i]);
            curr_node = Some(i);
        }
    }
    match curr_node {
        Some(i) => i as i32,
        None => -1,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::closest_meeting_node;
    #[test]
    fn run_tc1() {
        let edges = vec![2, 2, 3, -1];
        let node1 = 0;
        let node2 = 1;
        assert_eq!(closest_meeting_node(edges, node1, node2), 2);
    }
    #[test]
    fn run_tc2() {
        let edges = vec![1, 2, -1];
        let node1 = 0;
        let node2 = 2;
        assert_eq!(closest_meeting_node(edges, node1, node2), 2);
    }
    #[test]
    fn run_tc3() {
        let edges = vec![5, 4, 5, 4, 3, 6, -1];
        let node1 = 0;
        let node2 = 1;
        assert_eq!(closest_meeting_node(edges, node1, node2), -1);
    }
}
fn main() {
    let edges = vec![2, 2, 3, -1];
    let node1 = 0;
    let node2 = 1;
    assert_eq!(closest_meeting_node(edges, node1, node2), 2);
}
