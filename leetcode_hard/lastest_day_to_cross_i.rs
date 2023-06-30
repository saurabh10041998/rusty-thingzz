use std::collections::VecDeque;
struct Cell {
    x: usize,
    y: usize,
}

impl From<&Vec<i32>> for Cell {
    fn from(val: &Vec<i32>) -> Self {
        assert_eq!(val.len(), 2);
        Cell {
            x: val[0] as usize,
            y: val[1] as usize,
        }
    }
}

fn checked_add_signed(x: isize, y: isize) -> Option<usize> {
    if x - y < 0 {
        return None;
    }
    Some((x - y) as usize)
}

fn check(row: usize, col: usize, day: usize, cells: &[Vec<i32>]) -> bool {
    let mut grid = vec![vec![0; col]; row];
    for cell in cells.iter().take(day) {
        let c: Cell = cell.into();
        grid[c.x - 1][c.y - 1] = 1;
    }
    let mut q = VecDeque::new();
    for c in 0..col {
        if grid[0][c] == 0 {
            q.push_back((0usize, c));
            grid[0][c] = 1;
        }
    }
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((x, y)) = q.pop_front() {
        if x == row - 1 {
            return true;
        }
        for &(dx, dy) in directions.iter() {
            match (
                checked_add_signed(x as isize, dx),
                checked_add_signed(y as isize, dy),
            ) {
                (None, _) => {}
                (_, None) => {}
                (Some(new_x), Some(new_y)) => {
                    if new_x < row && new_y < col && grid[new_x][new_y] == 0 {
                        q.push_back((new_x, new_y));
                        grid[new_x][new_y] = 1;
                    }
                }
            }
        }
    }
    false
}

fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    let mut low = 1;
    let mut high = cells.len();
    let mut ans = 1;
    while low <= high {
        let offset = match high.checked_sub(low) {
            Some(x) => x / 2,
            None => break,
        };
        let mid = low + offset;
        if check(row as usize, col as usize, mid, &cells) {
            ans = mid;
            low = mid + 1;
        } else {
            high = match mid.checked_sub(1) {
                Some(x) => x,
                None => break,
            };
        }
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
