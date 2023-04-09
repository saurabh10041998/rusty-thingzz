use std::collections::HashSet;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Eq, PartialEq)]
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
            _ => panic!("[!!] Current character is not present in charset"),
        }
    }
}

impl IndexMut<char> for CharVec {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'a'..='z' => &mut self.0[(index as u8 - 'a' as u8) as usize],
            _ => panic!("[!!] Current character is not present in charset"),
        }
    }
}

fn dfs_helper(
    node: usize,
    visited: &mut HashSet<usize>,
    path: &mut HashSet<usize>,
    adj_lst: &Vec<Vec<usize>>,
    count: &mut Vec<CharVec>,
    colors: &Vec<char>,
) -> Option<i32> {
    if path.contains(&node) {
        // Cycle detection
        return None;
    }
    if visited.contains(&node) {
        // Already processed
        return Some(0);
    }
    visited.insert(node);
    path.insert(node);

    // Update the freq
    count[node][colors[node]] = 1;

    for &nei in adj_lst[node].iter() {
        match dfs_helper(nei, visited, path, adj_lst, count, colors) {
            Some(_) => {}
            None => {
                return None;
            }
        }
        for c in 'a'..='z' {
            count[node][c] = i32::max(
                count[node][c],
                count[nei][c] + if c == colors[node] { 1 } else { 0 },
            );
        }
    }
    path.remove(&node);
    Some(*count[node].0.iter().max().unwrap())
}

fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
    let n = colors.len();
    let colors = colors.chars().collect::<Vec<char>>();
    let mut adj_lst = vec![vec![]; n];
    for e in edges {
        let (u, v) = (e[0] as usize, e[1] as usize);
        adj_lst[u].push(v);
    }
    let mut count = vec![];
    count.resize_with(n, || CharVec::new());
    let mut visited = HashSet::new();
    let mut path = HashSet::new();
    let mut res = 0;
    for i in 0..n {
        match dfs_helper(i, &mut visited, &mut path, &adj_lst, &mut count, &colors) {
            Some(ans) => {
                res = i32::max(res, ans);
            }
            None => {
                return -1;
            }
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::largest_path_value;
    #[test]
    fn run_tc1() {
        let colors = String::from("abaca");
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(largest_path_value(colors, edges), 3);
    }
    #[test]
    fn run_tc2() {
        let colors = String::from("a");
        let edges = vec![vec![0, 0]];
        assert_eq!(largest_path_value(colors, edges), -1);
    }
}

fn main() {
    let colors = String::from("abaca");
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]];
    assert_eq!(largest_path_value(colors, edges), 3);
}
