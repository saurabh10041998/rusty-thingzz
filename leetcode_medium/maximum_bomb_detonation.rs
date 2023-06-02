use std::collections::HashMap;
use std::collections::HashSet;

struct Bomb {
    x: i32,
    y: i32,
    r: i32,
}

impl From<&Vec<i32>> for Bomb {
    fn from(value: &Vec<i32>) -> Self {
        assert_eq!(value.len(), 3);
        Bomb {
            x: value[0],
            y: value[1],
            r: value[2],
        }
    }
}

fn dfs(adj_lst: &HashMap<usize, Vec<usize>>, src: usize, visited: &mut HashSet<usize>) {
    if visited.contains(&src) {
        return;
    }
    visited.insert(src);
    match adj_lst.get(&src) {
        Some(neighbors) => {
            for nei in neighbors {
                if !visited.contains(nei) {
                    dfs(adj_lst, *nei, visited);
                }
            }
        }
        None => {}
    }
}

fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    let mut adj_lst = HashMap::new();
    let n = bombs.len();
    for i in 0..n {
        for j in i + 1..n {
            let b1: Bomb = (&bombs[i]).into();
            let b2: Bomb = (&bombs[j]).into();
            let d = i64::pow((b1.x - b2.x) as i64, 2) + i64::pow((b1.y - b2.y) as i64, 2);
            if d <= i64::pow(b1.r as i64, 2) {
                adj_lst
                    .entry(i)
                    .and_modify(|v: &mut Vec<usize>| v.push(j))
                    .or_insert(vec![j]);
            }
            if d <= i64::pow(b2.r as i64, 2) {
                adj_lst
                    .entry(j)
                    .and_modify(|v| v.push(i))
                    .or_insert(vec![i]);
            }
        }
    }
    let mut res = 0;
    for i in 0..n {
        let mut visited = HashSet::new();
        dfs(&adj_lst, i, &mut visited);
        res = i32::max(res, visited.len() as i32);
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::maximum_detonation;
    #[test]
    fn run_tc1() {
        let bombs = vec![vec![2, 1, 3], vec![6, 1, 4]];
        assert_eq!(maximum_detonation(bombs), 2);
    }
    #[test]
    fn run_tc2() {
        let bombs = vec![vec![1, 1, 5], vec![10, 10, 5]];
        assert_eq!(maximum_detonation(bombs), 1);
    }
    #[test]
    fn run_tc3() {
        let bombs = vec![
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 3],
            vec![5, 6, 4],
        ];
        assert_eq!(maximum_detonation(bombs), 5);
    }
}

fn main() {
    let bombs = vec![
        vec![1, 2, 3],
        vec![2, 3, 1],
        vec![3, 4, 2],
        vec![4, 5, 3],
        vec![5, 6, 4],
    ];
    assert_eq!(maximum_detonation(bombs), 5);
}
