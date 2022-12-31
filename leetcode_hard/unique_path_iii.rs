use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Point { x: -1, y: -1 }
    }
    fn make_point(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn dfs(map: &mut HashMap<(i32, i32), i32>, pt: Point, zeros: &mut i32) -> i32 {
    if map.get(&(pt.x, pt.y)).is_none() || map.get(&(pt.x, pt.y)) == Some(&-1) {
        return 0;
    }
    if *map.get(&(pt.x, pt.y)).unwrap() == 2 {
        println!("{:?} {}", pt, *zeros);
        if *zeros == -1 {
            return 1;
        } else {
            return 0;
        }
    }
    let (x, y) = (pt.x, pt.y);
    map.entry((x, y)).and_modify(|v| *v = -1);
    *zeros -= 1;
    let total_paths = dfs(map, Point::make_point(x + 1, y), zeros)
        + dfs(map, Point::make_point(x - 1, y), zeros)
        + dfs(map, Point::make_point(x, y + 1), zeros)
        + dfs(map, Point::make_point(x, y - 1), zeros);
    map.entry((x, y)).and_modify(|v| *v = 0);
    *zeros += 1;
    return total_paths;
}

fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut map = HashMap::new();
    let mut zeros = 0;
    let mut start = Point::new();
    for i in 0..n {
        for j in 0..m {
            map.insert((i as i32, j as i32), grid[i][j]);
            if grid[i][j] == 0 {
                zeros += 1;
            } else if grid[i][j] == 1 {
                start.x = i as i32;
                start.y = j as i32;
            }
        }
    }
    println!("{:?}", map);
    dfs(&mut map, start, &mut zeros)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
        assert_eq!(unique_paths_iii(grid), 2);
    }
    #[test]
    fn run_tc2() {
        let grid = vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,0,2]];
        assert_eq!(unique_paths_iii(grid), 4);
    }
    #[test]
    fn run_tc3() {
        let grid = vec![vec![0,1],vec![2,0]];
        assert_eq!(unique_paths_iii(grid), 0);
    }
}

fn main() {
    let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
    assert_eq!(unique_paths_iii(grid), 2);
}
