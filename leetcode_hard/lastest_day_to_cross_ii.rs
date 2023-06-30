use std::collections::HashMap;
use std::collections::HashSet;

struct Cell {
    x: i32,
    y: i32,
}

impl From<Vec<i32>> for Cell {
    fn from(value: Vec<i32>) -> Self {
        assert_eq!(value.len(), 2);
        Cell {
            x: value[0] - 1,
            y: value[1] - 1,
        }
    }
}

struct DisjointSet {
    parents: HashMap<(i32, i32), (i32, i32)>,
}

impl DisjointSet {
    fn new(row: i32, col: i32) -> Self {
        let mut hs = HashMap::new();
        for i in 0..row {
            for j in 0..col {
                hs.entry((i, j)).or_insert((i, j));
            }
        }
        // insert left and right special coordinate
        hs.entry((-1, -1)).or_insert((-1, -1));
        hs.entry((-2, -2)).or_insert((-2, -2));
        DisjointSet { parents: hs }
    }

    fn find_parent(&mut self, x: (i32, i32)) -> (i32, i32) {
        match self.parents.get(&x) {
            Some(&parent) if parent == x => parent,
            Some(&parent) => self.find_parent(parent),
            None => unreachable!("[*] No parent found for: {:?}", x),
        }
    }

    fn union(&mut self, x: (i32, i32), y: (i32, i32)) {
        let par_x = self.find_parent(x);
        let par_y = self.find_parent(y);
        self.parents.entry(par_x).and_modify(|y| *y = par_y);
    }
}

fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut ds = DisjointSet::new(row, col);
    let left: (i32, i32) = (-1, -1);
    let right: (i32, i32) = (-2, -2);
    let mut seen = HashSet::new();
    let ans = cells.len();
    for (i, v) in cells.into_iter().enumerate() {
        let c: Cell = v.into();
        for &(dx, dy) in directions.iter() {
            let new_x = c.x + dx;
            let new_y = c.y + dy;
            if new_x >= 0 && new_x < row {
                if new_y >= 0 && new_y < col {
                    if seen.contains(&(new_x, new_y)) {
                        ds.union((c.x, c.y), (new_x, new_y))
                    }
                } else if new_y == -1 {
                    ds.union((c.x, c.y), left);
                } else if new_y == col {
                    ds.union((c.x, c.y), right);
                }
            }
            if ds.find_parent(left) == ds.find_parent(right) {
                return i as i32;
            }
        }
        seen.insert((c.x, c.y));
    }
    ans as i32
}

#[cfg(test)]
pub mod tests {
    use crate::latest_day_to_cross;
    #[test]
    fn run_tc1() {
        let row = 2;
        let col = 2;
        let cells = vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]];
        assert_eq!(latest_day_to_cross(row, col, cells), 2);
    }
    #[test]
    fn run_tc2() {
        let row = 2;
        let col = 2;
        let cells = vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]];
        assert_eq!(latest_day_to_cross(row, col, cells), 1);
    }
    #[test]
    fn run_tc3() {
        let row = 3;
        let col = 3;
        let cells = vec![
            vec![1, 2],
            vec![2, 1],
            vec![3, 3],
            vec![2, 2],
            vec![1, 1],
            vec![1, 3],
            vec![2, 3],
            vec![3, 2],
            vec![3, 1],
        ];
        assert_eq!(latest_day_to_cross(row, col, cells), 3);
    }
}

fn main() {
    let row = 3;
    let col = 3;
    let cells = vec![
        vec![1, 2],
        vec![2, 1],
        vec![3, 3],
        vec![2, 2],
        vec![1, 1],
        vec![1, 3],
        vec![2, 3],
        vec![3, 2],
        vec![3, 1],
    ];
    assert_eq!(latest_day_to_cross(row, col, cells), 3);
}
