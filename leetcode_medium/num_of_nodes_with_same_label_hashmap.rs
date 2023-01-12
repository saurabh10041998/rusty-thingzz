use std::collections::HashMap;
use std::collections::HashSet;

fn dfs(
    adj: &Vec<HashSet<usize>>,
    visited: &mut HashSet<usize>,
    char_maps: &mut Vec<HashMap<char, i32>>,
    labels: &Vec<char>,
    src: usize,
) {
    if visited.contains(&src) {
        return;
    }
    visited.insert(src);
    char_maps[src]
        .entry(labels[src])
        .and_modify(|v| *v += 1)
        .or_insert(1);
    for u in adj[src].iter() {
        if !visited.contains(u) {
            dfs(adj, visited, char_maps, labels, *u);
            for (c, f) in char_maps[*u].clone().iter() {
                char_maps[src]
                    .entry(*c)
                    .and_modify(|v| *v += *f)
                    .or_insert(*f);
            }
        }
    }
}

fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
    let mut adj = vec![HashSet::new(); n as usize];
    for e in edges.iter() {
        let (u, v) = (e[0] as usize, e[1] as usize);
        adj[u].insert(v);
        adj[v].insert(u);
    }
    let labels = labels.chars().collect::<Vec<char>>();
    let mut visited = HashSet::new();
    let mut char_map = vec![HashMap::new(); n as usize];
    dfs(&adj, &mut visited, &mut char_map, &labels, 0);
    let mut ans = vec![];
    for (i, c) in char_map.iter().enumerate() {
        ans.push(*c.get(&labels[i]).unwrap())
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::count_sub_trees;
    #[test]
    fn run_tc1() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let labels = String::from("abaedcd");
        assert_eq!(count_sub_trees(n, edges, labels), vec![2, 1, 1, 1, 1, 1, 1])
    }
    #[test]
    fn run_tc2() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 3]];
        let labels = String::from("bbbb");
        assert_eq!(count_sub_trees(n, edges, labels), vec![4, 2, 1, 1])
    }
}

fn main() {
    let n = 7;
    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6],
    ];
    let labels = String::from("abaedcd");
    assert_eq!(count_sub_trees(n, edges, labels), vec![2, 1, 1, 1, 1, 1, 1])
}
