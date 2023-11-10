use std::collections::{HashMap, VecDeque};

fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut adj_lst = HashMap::new();
    let mut visited = HashMap::new();
    for pair in adjacent_pairs {
        adj_lst
            .entry(pair[0])
            .and_modify(|lst: &mut Vec<i32>| lst.push(pair[1]))
            .or_insert(vec![pair[1]]);
        adj_lst
            .entry(pair[1])
            .and_modify(|lst| lst.push(pair[0]))
            .or_insert(vec![pair[0]]);
        visited.entry(pair[0]).or_insert(false);
        visited.entry(pair[1]).or_insert(false);
    }
    let mut root = None;
    // Find the root of the tree
    for (k, v) in adj_lst.iter() {
        if v.len() == 1 {
            root = Some(k);
            break;
        }
    }

    let first = match root {
        Some(val) => val,
        None => unreachable!(),
    };

    let mut q = VecDeque::new();
    q.push_back(*first);
    visited.entry(*first).and_modify(|status| *status = true);

    let mut ans = vec![];

    while let Some(ele) = q.pop_front() {
        ans.push(ele);
        if let Some(neighbours) = adj_lst.get(&ele) {
            for n in neighbours {
                match visited.get_mut(n) {
                    Some(val) if !*val => {
                        *val = true;
                        q.push_back(*n);
                    }
                    _ => {}
                }
            }
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::restore_array;

    pub fn verify(res: Vec<i32>, ans: Vec<Vec<i32>>) -> bool {
        for a in ans {
            if a == res {
                return true;
            }
        }
        false
    }
    #[test]
    fn run_tc1() {
        let adjacent_pairs = vec![vec![2, 1], vec![3, 4], vec![3, 2]];
        let ans = vec![vec![1, 2, 3, 4], vec![4, 3, 2, 1]];
        let res = restore_array(adjacent_pairs);
        assert!(verify(res, ans));
    }
    #[test]
    fn run_tc2() {
        let adjacent_pairs = vec![vec![4, -2], vec![1, 4], vec![-3, 1]];
        let ans = vec![vec![-2, 4, 1, -3], vec![-3, 1, 4, -2]];
        let res = restore_array(adjacent_pairs);
        assert!(verify(res, ans));
    }
    #[test]
    fn run_tc3() {
        let adjacent_pairs = vec![vec![100000, -100000]];
        let ans = vec![vec![100000, -100000], vec![-100000, 100000]];
        let res = restore_array(adjacent_pairs);
        assert!(verify(res, ans));
    }
}

fn main() {
    let adjacent_pairs = vec![vec![100000, -100000]];
    let res = restore_array(adjacent_pairs);
    dbg!(res);
}
