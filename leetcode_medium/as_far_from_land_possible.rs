use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn bfs(map: &HashMap<(i32, i32), i32>, mut srcs: HashSet<(i32, i32)>) -> i32 {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.extend(srcs.drain());
    let mut dist = 0;
    while !q.is_empty() {
        dist += 1;
        let dir = [(0, -1), (0, 1), (1, 0), (-1, 0)];
        let mut n = q.len() as i32;
        while n > 0 {
            let (x, y) = q.pop_front().unwrap();
            for (dx, dy) in dir.iter() {
                let new_pt = (x + dx, y + dy);
                match map.get(&new_pt) {
                    Some(v) if *v == 0 => {
                        if !visited.contains(&new_pt) {
                            q.push_back(new_pt);
                            visited.insert(new_pt);
                        }
                    }
                    Some(v) => {
                        assert_eq!(*v, 1);
                    }
                    None => {}
                }
            }
            n -= 1;
        }
    }
    // we consider all land on level 1
    // so dist = level - 1
    dist - 1
}

fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::new();
    let mut srcs = HashSet::new();
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in 0..m {
            map.entry((i as i32, j as i32)).or_insert(grid[i][j]);
            if grid[i][j] == 1 {
                srcs.insert((i as i32, j as i32));
            }
        }
    }
    if srcs.len() == 0 || srcs.len() == m * n {
        return -1;
    }
    bfs(&map, srcs)
}

#[cfg(test)]
pub mod tests {
    use crate::max_distance;
    #[test]
    fn run_tc1() {
        let grid = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(max_distance(grid), 2);
    }
    #[test]
    fn run_tc2() {
        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(max_distance(grid), 4);
    }
    #[test]
    fn run_tc3() {
        let grid = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(max_distance(grid), -1);
    }
}

fn main() {
    let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(max_distance(grid), 4);
}
