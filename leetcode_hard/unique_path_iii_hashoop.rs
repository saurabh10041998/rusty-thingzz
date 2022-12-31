use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct Splitmix64 {}

impl Splitmix64 {
    fn splitmix64(mut x: u64) -> u64 {
        x += 0x9e3779b97f4a7c15;
        x = (x ^ (x >> 30)) * 0xbf58476d1ce4e5b9;
        x = (x ^ (x >> 27)) * 0x94d049bb133111eb;
        return x ^ (x >> 31);
    }
}

pub struct CustomHasher {
    state: u64,
}

impl Hasher for CustomHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes.iter().rev() {
            self.state = self.state.rotate_left(8) ^ u64::from(byte);
        }
        let fixed_random = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        self.state = Splitmix64::splitmix64(self.state + fixed_random);
    }
    fn finish(&self) -> u64 {
        self.state
    }
}

pub struct BuildCustomHasher;

impl BuildHasher for BuildCustomHasher {
    type Hasher = CustomHasher;
    fn build_hasher(&self) -> Self::Hasher {
        CustomHasher { state: 0 }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x, self.y).hash(state);
    }
}

fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let (n, m) = (grid.len(), grid[0].len());
    let mut zeros = 0;
    let mut start = Point::new(-1, -1);
    // Using the custom hasher
    let mut map = HashMap::with_hasher(BuildCustomHasher);
    for i in 0..n {
        for j in 0..m {
            map.insert(Point::new(i as i32, j as i32), grid[i][j]);
            if grid[i][j] == 0 {
                zeros += 1;
            } else if grid[i][j] == 1 {
                start = Point::new(i as i32, j as i32);
            }
        }
    }
    dfs(&mut map, start, &mut zeros)
}

fn dfs(map: &mut HashMap<Point, i32, BuildCustomHasher>, pt: Point, zeros: &mut i32) -> i32 {
    if map.get(&pt).is_none() || map.get(&pt) == Some(&(-1)) {
        return 0;
    }
    if *map.get(&pt).unwrap() == 2 {
        if *zeros == -1 {
            return 1;
        } else {
            return 0;
        }
    }
    let (x, y) = (pt.x, pt.y);
    map.entry(pt.clone()).and_modify(|v| *v = -1);
    *zeros -= 1;
    let total_paths = dfs(map, Point::new(x - 1, y), zeros)
        + dfs(map, Point::new(x + 1, y), zeros)
        + dfs(map, Point::new(x, y - 1), zeros)
        + dfs(map, Point::new(x, y + 1), zeros);
    map.entry(pt).and_modify(|v| *v = 0);
    *zeros += 1;
    return total_paths;
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
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]];
        assert_eq!(unique_paths_iii(grid), 4);
    }
    #[test]
    fn run_tc3() {
        let grid = vec![vec![0, 1], vec![2, 0]];
        assert_eq!(unique_paths_iii(grid), 0);
    }
}

fn main() {
    let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
    assert_eq!(unique_paths_iii(grid), 2);
}
