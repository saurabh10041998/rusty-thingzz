use std::collections::HashMap;
use std::collections::HashSet;
pub struct DisjointSet {
    parent: Vec<i32>,
}

impl DisjointSet {
    fn new() -> Self {
        DisjointSet { parent: Vec::new() }
    }

    fn init_parent(&mut self, k: usize) {
        for i in 0..k {
            self.parent.push(i as i32);
        }
    }

    fn find_parent(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] == x {
            return x;
        }
        self.parent[x as usize] = self.find_parent(self.parent[x as usize]);
        self.parent[x as usize]
    }

    fn union_by_rank(&mut self, x: i32, y: i32) -> bool {
        let p_x = self.find_parent(x);
        let p_y = self.find_parent(y);
        if p_x == p_y {
            return false;
        }
        if p_x > p_y {
            self.parent[p_y as usize] = p_x;
        } else {
            // p_x < p_y
            self.parent[p_x as usize] = p_y;
        }
        true
    }
}

fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let (mut row, mut cols) = (HashMap::new(), HashMap::new());
    let mut ds = DisjointSet::new();
    ds.init_parent(stones.len());
    for (k, v) in stones.iter().enumerate() {
        let (mut par_i, mut par_j) = (i32::MAX, i32::MAX);
        let (i, j) = (v.first().unwrap(), v.last().unwrap());
        if !row.contains_key(i) {
            row.insert(i, k);
        } else {
            par_i = row.get(i).unwrap().to_owned() as i32;
        }
        if !cols.contains_key(j) {
            cols.insert(j, k);
        } else {
            par_j = cols.get(j).unwrap().to_owned() as i32;
        }
        if par_i != i32::MAX {
            ds.union_by_rank(par_i, k as i32);
        }
        if par_j != i32::MAX {
            ds.union_by_rank(par_j, k as i32);
        }
    }
    let mut ans = 0;
    let mut uniq_parent = HashSet::new();
    for (k, _) in stones.iter().enumerate() {
        let par_i = ds.find_parent(k as i32);
        if !uniq_parent.contains(&par_i) {
            ans += 1;
            uniq_parent.insert(par_i);
        }
    }
    stones.len() as i32 - ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let stones: Vec<Vec<i32>> = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ];
        assert_eq!(remove_stones(stones), 5);
    }
    #[test]
    fn run_tc2() {
        let stones: Vec<Vec<i32>> =
            vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]];
        assert_eq!(remove_stones(stones), 3);
    }

    #[test]
    fn run_tc3() {
        let stones: Vec<Vec<i32>> = vec![vec![0, 0]];
        assert_eq!(remove_stones(stones), 0);
    }
}

fn main() {
    println!("Hello, world!");
    let stones: Vec<Vec<i32>> = vec![
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![1, 2],
        vec![2, 1],
        vec![2, 2],
    ];
    assert_eq!(remove_stones(stones), 5);
}
