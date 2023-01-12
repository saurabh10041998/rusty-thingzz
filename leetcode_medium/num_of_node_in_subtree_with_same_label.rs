use std::collections::HashSet;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct CharVec(Vec<i32>);

impl CharVec {
    fn new() -> Self {
        CharVec(vec![0; 26])
    }
}

impl Index<char> for CharVec {
    type Output = i32;
    fn index(&self, index: char) -> &Self::Output {
        match index {
            'a'..='z' => &self.0[(index as u8 - 'a' as u8) as usize],
            _ => panic!("[!!] Not supported in the current charset!!"),
        }
    }
}

impl IndexMut<char> for CharVec {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'a'..='z' => &mut self.0[(index as u8 - 'a' as u8) as usize],
            _ => panic!("[!!] Not suppored in the current charset!!"),
        }
    }
}

fn dfs(
    adj: &Vec<HashSet<usize>>,
    visited: &mut HashSet<usize>,
    char_maps: &mut Vec<CharVec>,
    labels: &Vec<char>,
    src: usize,
) {
    if visited.contains(&src) {
        return;
    }
    visited.insert(src);
    char_maps[src][labels[src]] += 1;
    for u in adj[src].iter() {
        if !visited.contains(u) {
            dfs(adj, visited, char_maps, labels, *u);
            for c in 'a'..='z' {
                char_maps[src][c] += char_maps[*u][c];
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
    let mut char_map = vec![];
    char_map.resize_with(n as usize, || CharVec::new());
    dfs(&adj, &mut visited, &mut char_map, &labels, 0);
    let mut ans = vec![];
    for (i, c) in char_map.iter().enumerate() {
        ans.push(c[labels[i]]);
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
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![0, 3],
        ];
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
