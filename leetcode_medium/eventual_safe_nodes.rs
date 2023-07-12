use std::collections::VecDeque;
fn eventual_safe_node(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let n = graph.len();
    let mut inverted_graph = vec![vec![]; n];
    let mut incomming_edges = vec![0; n];
    for (start_edge, end_edges) in graph.into_iter().enumerate() {
        for end_edge in end_edges {
            let end_edge_usize = end_edge as usize;
            inverted_graph[end_edge_usize].push(start_edge);
            incomming_edges[start_edge] += 1;
        }
    }
    let mut safe_nodes = vec![];
    let mut safe_nodes_q = VecDeque::new();
    for (idx, cnt) in incomming_edges.iter().enumerate() {
        if *cnt != 0 {
            continue;
        }
        safe_nodes_q.push_back(idx);
        safe_nodes.push(idx as i32);
    }
    while let Some(node) = safe_nodes_q.pop_front() {
        for end_edge in inverted_graph[node].iter() {
            incomming_edges[*end_edge] -= 1;
            if incomming_edges[*end_edge] == 0 {
                safe_nodes_q.push_back(*end_edge);
                safe_nodes.push(*end_edge as i32);
            }
        }
    }
    safe_nodes.sort();
    safe_nodes
}

#[cfg(test)]
pub mod tests {
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
