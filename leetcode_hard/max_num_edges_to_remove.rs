#[derive(Debug, PartialEq, Eq)]
enum Type {
    Alice,
    Bob,
    Both,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    typ: Type,
    src: usize,
    dst: usize,
}

impl From<Vec<i32>> for Edge {
    fn from(buffer: Vec<i32>) -> Self {
        let typ: Type = if buffer[0] == 1 {
            Type::Alice
        } else if buffer[0] == 2 {
            Type::Bob
        } else {
            Type::Both
        };
        Edge {
            typ,
            src: (buffer[1] - 1) as usize,
            dst: (buffer[2] - 1) as usize,
        }
    }
}

pub struct DisjointSet {
    _n: usize,
    parent: Vec<usize>,
    rank: Vec<i32>,
    count: usize,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let mut rank = vec![0; n];
        for i in 0..n {
            parent[i] = i;
            rank[i] = 1;
        }
        DisjointSet {
            _n: n,
            parent,
            rank,
            count: n,
        }
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
        self.count -= 1;
        true
    }
}

fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut dsa = DisjointSet::new(n as usize);
    let mut dsb = DisjointSet::new(n as usize);
    let mut both_edge_lst = Vec::new();
    let mut alice_edge_lst = Vec::new();
    let mut bob_edge_lst = Vec::new();
    for e in edges {
        let e: Edge = e.into();
        match e.typ {
            Type::Both => both_edge_lst.push(e),
            Type::Alice => alice_edge_lst.push(e),
            Type::Bob => bob_edge_lst.push(e),
        }
    }
    let mut cnt = 0;
    for e in both_edge_lst {
        if dsa.union(e.src, e.dst) {
            dsb.union(e.src, e.dst);
        } else {
            cnt += 1;
        }
    }
    for e in alice_edge_lst {
        if dsa.union(e.src, e.dst) {
        } else {
            cnt += 1;
        }
    }
    for e in bob_edge_lst {
        if dsb.union(e.src, e.dst) {
        } else {
            cnt += 1;
        }
    }
    if dsa.count == 1 && dsb.count == 1 {
        return cnt;
    } else {
        -1
    }
}

#[cfg(test)]
pub mod tests {
    use crate::max_num_edges_to_remove;
    #[test]
    fn run_tc1() {
        let n = 4;
        let edges = vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 3],
            vec![1, 2, 4],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ];
        assert_eq!(max_num_edges_to_remove(n, edges), 2);
    }
    #[test]
    fn run_tc2() {
        let n = 4;
        let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]];
        assert_eq!(max_num_edges_to_remove(n, edges), 0);
    }
    #[test]
    fn run_tc3() {
        let n = 4;
        let edges = vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]];
        assert_eq!(max_num_edges_to_remove(n, edges), -1);
    }

    #[test]
    fn run_tc4() {
        let n = 2;
        let edges = vec![vec![1, 1, 2], vec![2, 1, 2], vec![3, 1, 2]];
        assert_eq!(max_num_edges_to_remove(n, edges), 2);
    }
}

fn main() {
    let n = 4;
    let edges = vec![
        vec![3, 1, 2],
        vec![3, 2, 3],
        vec![1, 1, 3],
        vec![1, 2, 4],
        vec![1, 1, 2],
        vec![2, 3, 4],
    ];
    assert_eq!(max_num_edges_to_remove(n, edges), 2);
}
