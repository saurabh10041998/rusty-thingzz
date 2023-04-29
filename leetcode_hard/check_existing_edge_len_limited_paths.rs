use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub struct DisjointSet {
    n: usize,
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let mut rank = vec![0; n];
        for i in 0..n {
            parent[i] = i;
            rank[i] = 1;
        }
        DisjointSet { parent, rank, n }
    }

    fn find_parent(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find_parent(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let par_x = self.find_parent(x);
        let par_y = self.find_parent(y);
        if par_x == par_y {
            return false;
        }
        if self.rank[par_x] < self.rank[par_y] {
            self.rank[par_y] += self.rank[par_x];
            self.parent[par_x] = par_y;
        } else {
            self.rank[par_x] += self.rank[par_y];
            self.parent[par_y] = par_x;
        }
        true
    }
}

#[derive(PartialEq, Eq, Debug)]
struct EdgeList {
    src: usize,
    dst: usize,
    dist: i32,
}

impl From<Vec<i32>> for EdgeList {
    fn from(buffer: Vec<i32>) -> EdgeList {
        EdgeList {
            src: buffer[0] as usize,
            dst: buffer[1] as usize,
            dist: buffer[2],
        }
    }
}

impl PartialOrd for EdgeList {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for EdgeList {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Query {
    idx: usize,
    src: usize,
    dst: usize,
    limit: i32,
}

impl From<Vec<i32>> for Query {
    fn from(buffer: Vec<i32>) -> Self {
        Query {
            idx: buffer[0] as usize,
            src: buffer[1] as usize,
            dst: buffer[2] as usize,
            limit: buffer[3],
        }
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.limit.partial_cmp(&other.limit)
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.limit.cmp(&other.limit)
    }
}

fn distance_limited_paths_exist(
    n: i32,
    edge_list: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let mut ds = DisjointSet::new(n as usize);
    let mut edge_lst = edge_list
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<EdgeList>>();
    let mut queries = queries
        .into_iter()
        .enumerate()
        .map(|(idx, buffer)| vec![idx as i32, buffer[0], buffer[1], buffer[2]].into())
        .collect::<Vec<Query>>();
    edge_lst.sort();
    queries.sort();
    let mut res = vec![false; queries.len()];
    let mut i = 0;
    for q in queries {
        while i < edge_lst.len() && edge_lst[i].dist < q.limit {
            ds.union(edge_lst[i].src, edge_lst[i].dst);
            i += 1;
        }
        if ds.find_parent(q.src) == ds.find_parent(q.dst) {
            res[q.idx] = true;
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::distance_limited_paths_exist;
    #[test]
    fn run_tc1() {
        let n = 3;
        let edge_list = vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]];
        let queries = vec![vec![0, 1, 2], vec![0, 2, 5]];
        assert_eq!(
            distance_limited_paths_exist(n, edge_list, queries),
            vec![false, true]
        );
    }
    #[test]
    fn run_tc2() {
        let n = 5;
        let edge_list = vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]];
        let queries = vec![vec![0, 4, 14], vec![1, 4, 13]];
        assert_eq!(
            distance_limited_paths_exist(n, edge_list, queries),
            vec![true, false]
        );
    }
}

fn main() {
    let n = 3;
    let edge_list = vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]];
    let queries = vec![vec![0, 1, 2], vec![0, 2, 5]];
    assert_eq!(
        distance_limited_paths_exist(n, edge_list, queries),
        vec![false, true]
    );
}
