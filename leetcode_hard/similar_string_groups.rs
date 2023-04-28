#[derive(Debug, PartialEq, Eq)]
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<i32>,
    n: usize,
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
            parent,
            rank,
            n,
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

fn is_similar<'a>(a: &'a str, b: &'a str) -> bool {
    let mut diff = 0;
    for (x, y) in a.chars().zip(b.chars()) {
        if x != y {
            diff += 1;
        }
        if diff > 2 {
            return false;
        }
    }
    diff == 0 || diff == 2
}

fn num_similar_groups(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let mut ds = DisjointSet::new(n);
    for i in 0..n {
        for j in i + 1..n {
            if is_similar(&strs[i], &strs[j]) {
                ds.union(i, j);
            }
        }
    }
    ds.count as i32
}

#[cfg(test)]
pub mod tests {
    use crate::num_similar_groups;
    #[test]
    fn run_tc1() {
        let strs = vec![
            String::from("tars"),
            String::from("rats"),
            String::from("arts"),
            String::from("star"),
        ];
        assert_eq!(num_similar_groups(strs), 2);
    }
    #[test]
    fn run_tc2() {
        let strs = vec![String::from("ovm"), String::from("omv")];
        assert_eq!(num_similar_groups(strs), 1);
    }
}

fn main() {
    let strs = vec![
        String::from("tars"),
        String::from("rats"),
        String::from("arts"),
        String::from("star"),
    ];
    assert_eq!(num_similar_groups(strs), 2);
}
