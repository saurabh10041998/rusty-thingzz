pub struct DisjointSet {
    rank: Vec<usize>,
    parent: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut rank = vec![0usize; n];
        let mut parent = vec![0usize; n];
        for i in 0..n {
            rank[i] = 1;
            parent[i] = i;
        }
        DisjointSet { rank, parent }
    }

    fn find_parent(&mut self, n: usize) -> usize {
        if self.parent[n] != n {
            self.parent[n] = self.find_parent(self.parent[n]);
        }
        self.parent[n]
    }

    fn union(&mut self, x: usize, y: usize) {
        let par_x = self.find_parent(x);
        let par_y = self.find_parent(y);
        if par_x == par_y {
            return;
        }
        if self.rank[par_x] > self.rank[par_y] {
            self.rank[par_x] += self.rank[par_y];
            self.parent[par_y] = par_x;
        } else {
            self.rank[par_y] += self.rank[par_x];
            self.parent[par_x] = par_y;
        }
    }
}

fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let mut connected_pairs: i64 = 0;
    let mut ds = DisjointSet::new(n as usize);
    for e in edges {
        ds.union(e[0] as usize, e[1] as usize);
    }
    let mut list = Vec::new();
    for i in 0..n as usize {
        if ds.parent[i] == i {
            list.push(ds.rank[i] as i64);
        }
    }
    let total_pairs = ((n as i64) * (n - 1) as i64) / 2;
    for l in list {
        connected_pairs += (l * (l - 1)) / 2;
    }

    total_pairs - connected_pairs
}

#[cfg(test)]
pub mod tests {
    use crate::count_pairs;
    #[test]
    fn run_tc1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        assert_eq!(count_pairs(n, edges), 0);
    }
    #[test]
    fn run_tc2() {
        let n = 7;
        let edges = vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]];
        assert_eq!(count_pairs(n, edges), 14);
    }
}

fn main() {
    let n = 3;
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
    assert_eq!(count_pairs(n, edges), 0);
}
