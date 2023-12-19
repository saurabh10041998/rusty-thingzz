//! Implement the instruction in the question
//!
use std::collections::HashMap;

fn adjacent_9(x: isize, y: isize) -> Vec<(isize, isize)> {
    let mut adj = vec![];
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            adj.push((x + dx, y + dy))
        }
    }
    adj
}

fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut hs = HashMap::new();
    let n = img.len();
    let m = img[0].len();
    for y in 0..n {
        for x in 0..m {
            hs.entry((y as isize, x as isize)).or_insert(img[y][x]);
        }
    }
    let mut grid = vec![vec![0; m]; n];
    for y in 0..n {
        for x in 0..m {
            let mut cnt = 0;
            let mut sum = 0;
            for (new_x, new_y) in adjacent_9(x as isize, y as isize) {
                match hs.get(&(new_y, new_x)) {
                    Some(val) => {
                        sum += val;
                        cnt += 1;
                    }
                    None => {}
                }
            }
            grid[y][x] = sum / cnt;
        }
    }
    grid
}

#[cfg(test)]
pub mod tests {
    use crate::image_smoother;
    #[test]
    fn run_tc1() {
        let img = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert_eq!(
            image_smoother(img),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
    }
    #[test]
    fn run_tc2() {
        let img = vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]];
        assert_eq!(
            image_smoother(img),
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137]
            ]
        );
    }
}

fn main() {
    let img = vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]];
    assert_eq!(
        image_smoother(img),
        vec![
            vec![137, 141, 137],
            vec![141, 138, 141],
            vec![137, 141, 137]
        ]
    );
}
