use std::collections::HashSet;

fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let mut graph: Vec<HashSet<i32>> = vec![];
    graph.resize_with(n as usize, || HashSet::new());
    for e in edges.iter() {
        graph[(e[0] as i32 - 1) as usize].insert(e[1] - 1);
        graph[(e[1] as i32 - 1) as usize].insert(e[0] - 1);
    }
    let mut nodes = vec![];
    for u in 0..n {
        if graph[u as usize].len() % 2 != 0 {
            nodes.push(u);
        }
    }
    match nodes.len() {
        0 => true,
        2 => {
            for g in graph {
                if g.get(&nodes[0]).is_none() && g.get(&nodes[1]).is_none() {
                    return true;
                }
            }
            return false;
        }
        4 => {
            let (a, b, c, d) = (nodes[0], nodes[1], nodes[2], nodes[3]);
            if graph[a as usize].get(&b).is_none() && graph[c as usize].get(&d).is_none()
                || graph[a as usize].get(&c).is_none() && graph[b as usize].get(&d).is_none()
                || graph[a as usize].get(&d).is_none() && graph[b as usize].get(&c).is_none()
            {
                return true;
            }
            return false;
        }
        _ => false,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::is_possible;
    #[test]
    fn run_tc1() {
        let n = 5;
        let edges = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 2],
            vec![1, 4],
            vec![2, 5],
        ];
        assert_eq!(is_possible(n, edges), true);
    }

    #[test]
    fn run_tc2() {
        let n = 4;
        let edges = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(is_possible(n, edges), true);
    }

    #[test]
    fn run_tc3() {
        let n = 4;
        let edges = vec![vec![4, 1], vec![3, 2]];
        assert_eq!(is_possible(n, edges), true);
    }
}

fn main() {
    let n = 5;
    let edges = vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 2],
        vec![1, 4],
        vec![2, 5],
    ];
    assert_eq!(is_possible(n, edges), true);
}
