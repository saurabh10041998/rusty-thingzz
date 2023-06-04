struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let rank = vec![1; n];
        for i in 0..n {
            parent[i] = i;
        }
        DisjointSet { parent, rank }
    }

    fn find_parent(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find_parent(self.parent[x]);
        }
        self.parent[x]
    }

    fn union_by_rank(&mut self, x: usize, y: usize) {
        let par_x = self.find_parent(x);
        let par_y = self.find_parent(y);
        if par_x == par_y {
            return;
        }
        if self.rank[par_x] < self.rank[par_y] {
            self.rank[par_y] += self.rank[par_x];
            self.parent[par_x] = par_y;
        } else {
            self.rank[par_x] += self.rank[par_y];
            self.parent[par_y] = par_x;
        }
    }

    fn get_total_parents(&self) -> usize {
        let n = self.parent.len();
        let mut count = 0;
        for i in 0..n {
            if self.parent[i] == i {
                count += 1;
            }
        }
        count
    }
}

fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut ds = DisjointSet::new(n);
    for i in 0..n {
        for j in 0..n {
            if is_connected[i][j] == 1 {
                ds.union_by_rank(i, j);
            }
        }
    }
    ds.get_total_parents() as i32
}

#[cfg(test)]
pub mod tests {
    use crate::find_circle_num;
    #[test]
    fn run_tc1() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        assert_eq!(find_circle_num(is_connected), 2);
    }
    #[test]
    fn run_tc2() {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(find_circle_num(is_connected), 3);
    }
}

fn main() {
    let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    assert_eq!(find_circle_num(is_connected), 3);
}
