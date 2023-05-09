use std::collections::HashSet;

struct Matrix<'a> {
    matrix: &'a Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

impl<'a> Matrix<'a> {
    fn new(matrix: &'a Vec<Vec<i32>>) -> Self {
        Matrix {
            matrix,
            rows: matrix.len(),
            cols: matrix[0].len(),
        }
    }

    fn index(&self, i: usize, j: usize) -> i32 {
        self.matrix[i][j]
    }

    fn iter_spiral<T: Iterator<Item = (i32, i32)>>(&self, directions: T) -> SpiralIterator<T> {
        //let mut directions = vec![(0, 1), (1, 0), (0, -1),(-1, 0)].into_iter().cycle();
        SpiralIterator::new(self, directions)
    }
}

struct SpiralIterator<'a, T>
where
    T: Iterator<Item = (i32, i32)>,
{
    mat: &'a Matrix<'a>,
    x: usize,
    y: usize,
    val_left: usize,
    directions: T,
    dir: (i32, i32),
    visited: HashSet<(usize, usize)>,
}

impl<'a, T> SpiralIterator<'a, T>
where
    T: Iterator<Item = (i32, i32)>,
{
    fn new(mat: &'a Matrix<'a>, mut directions: T) -> Self {
        SpiralIterator {
            mat,
            x: 0,
            y: 0,
            val_left: mat.rows * mat.cols,
            dir: directions.nth(0).unwrap(),
            directions,
            visited: HashSet::new(),
        }
    }

    fn next_direction(&mut self) -> (i32, i32) {
        // Safe to call unwrap due to cyclic iterator
        self.directions.next().unwrap()
    }

    fn move_next(&mut self) {
        if self.val_left == 0 {
            return;
        }
        //println!("{:?}", self.dir);
        match self.make_move(self.x, self.y, self.dir) {
            Some((new_x, new_y)) => {
                self.x = new_x;
                self.y = new_y;
            }
            None => {
                self.dir = self.next_direction();
                self.move_next();
            }
        }
    }

    fn make_move(&self, i: usize, j: usize, dir: (i32, i32)) -> Option<(usize, usize)> {
        match (
            i.checked_add_signed(dir.0 as isize),
            j.checked_add_signed(dir.1 as isize),
        ) {
            (None, _) => None,
            (_, None) => None,
            (Some(new_i), Some(new_j)) => {
                if new_i < self.mat.rows && new_j < self.mat.cols {
                    if !self.visited.contains(&(new_i, new_j)) {
                        return Some((new_i, new_j));
                    } else {
                        return None;
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl<'a, T> Iterator for SpiralIterator<'a, T>
where
    T: Iterator<Item = (i32, i32)>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.val_left == 0 {
            return None;
        }
        self.val_left -= 1;
        let val = self.mat.index(self.x, self.y);
        self.visited.insert((self.x, self.y));
        self.move_next();
        Some(val)
    }
}

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter().cycle();
    Matrix::new(&matrix).iter_spiral(directions).collect()
}

#[cfg(test)]
pub mod tests {
    use crate::spiral_order;
    #[test]
    fn run_tc1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(spiral_order(matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }
    #[test]
    fn run_tc2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            spiral_order(matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(spiral_order(matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
}
