use std::collections::HashSet;

fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut degree = vec![0; n as usize];
    let mut adj_lst = vec![];
    adj_lst.resize_with(n as usize, HashSet::new);
    for r in roads {
        let (u, v) = (r[0] as usize, r[1] as usize);
        degree[u] += 1;
        degree[v] += 1;
        adj_lst[u].insert(v);
        adj_lst[v].insert(u);
    }
    let mut max_rank = 0;
    for i in 0..n as usize {
        for j in i + 1..n as usize {
            let mut rank = degree[i] + degree[j];
            rank = if adj_lst[i].contains(&j) {
                rank - 1
            } else {
                rank
            };
            max_rank = i32::max(max_rank, rank);
        }
    }
    max_rank
}

#[cfg(test)]
pub mod tests {
    use crate::maximal_network_rank;
    #[test]
    fn run_tc1() {
        let n = 4;
        let roads = vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]];
        assert_eq!(maximal_network_rank(n, roads), 4);
    }
    #[test]
    fn run_tc2() {
        let n = 5;
        let roads = vec![
            vec![0, 1],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![2, 4],
        ];
        assert_eq!(maximal_network_rank(n, roads), 5);
    }
    #[test]
    fn run_tc3() {
        let n = 8;
        let roads = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![2, 4],
            vec![5, 6],
            vec![5, 7],
        ];
        assert_eq!(maximal_network_rank(n, roads), 5);
    }
}

fn main() {
    let n = 5;
    let roads = vec![
        vec![0, 1],
        vec![0, 3],
        vec![1, 2],
        vec![1, 3],
        vec![2, 3],
        vec![2, 4],
    ];
    assert_eq!(maximal_network_rank(n, roads), 5);
}
