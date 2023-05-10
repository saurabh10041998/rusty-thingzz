use std::collections::HashSet;

struct Dimension {
    rows: usize,
    cols: usize
}

impl From<(usize, usize)> for Dimension {
    fn from(buffer: (usize, usize)) -> Self {
        Dimension {
            rows: buffer.0,
            cols: buffer.1
        }
    }
}

struct SpiralIterator<T>
where
    T: Iterator<Item = (i32, i32)>,
{
    directions: T,
    start: usize,
    x: usize,
    y: usize,
    dir: (i32, i32),
    vals_left: usize,
    dim: Dimension,
    visited: HashSet<(usize, usize)>,
}

impl<T> SpiralIterator<T>
where
    T: Iterator<Item = (i32, i32)>,
{
    fn new(mut directions: T, dim: Dimension) -> Self {
        SpiralIterator {
            dir: directions.nth(0).unwrap(),
            directions,
            x: 0,
            y: 0,
            vals_left: dim.rows * dim.cols,
            dim,
            start: 1,
            visited: HashSet::new(),
        }
    }
    fn get_next_direction(&mut self) -> (i32, i32) {
        self.directions.next().unwrap()
    }
    fn move_next(&mut self) {
        if self.vals_left == 0 {
            return;
        }
        match self.make_move(self.x, self.y, self.dir) {
            Some((new_x, new_y)) => {
                self.x = new_x;
                self.y = new_y;
            }
            None => {
                self.dir = self.get_next_direction();
                self.move_next();
            }
        }
    }

    fn make_move(&self, x: usize, y: usize, dir: (i32, i32)) -> Option<(usize, usize)> {
        match (
            x.checked_add_signed(dir.0 as isize),
            y.checked_add_signed(dir.1 as isize),
        ) {
            (None, _) => None,
            (_, None) => None,
            (Some(new_x), Some(new_y)) => {
                if new_x < self.dim.rows && new_y < self.dim.cols {
                    if !self.visited.contains(&(new_x, new_y)) {
                        Some((new_x, new_y))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl<T> Iterator for SpiralIterator<T>
where
    T: Iterator<Item = (i32, i32)>,
{
    type Item = (usize, usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.vals_left == 0 {
            return None;
        }
        self.vals_left -= 1;
        let val = self.start;
        let i = self.x;
        let j = self.y;
        self.visited.insert((self.x, self.y));
        self.start += 1;
        self.move_next();
        Some((i, j, val))
    }
}

fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; n as usize]; n as usize];
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter().cycle();
    SpiralIterator::new(directions, (n as usize, n as usize).into())
        .for_each(|x| grid[x.0][x.1] = x.2 as i32);
    grid
}

#[cfg(test)]
pub mod tests {
    use crate::generate_matrix;
    #[test]
    fn run_tc1() {
        let n = 3;
        assert_eq!(
            generate_matrix(n),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }
    #[test]
    fn run_tc2() {
        let n = 1;
        assert_eq!(generate_matrix(n), vec![vec![1]]);
    }
}

fn main() {
    let n = 3;
    assert_eq!(
        generate_matrix(n),
        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    );
}
