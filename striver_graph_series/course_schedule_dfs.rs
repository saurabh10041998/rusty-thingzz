use std::collections::HashMap;
use std::collections::HashSet;

fn detect_cycle(
    src: usize,
    adj: &HashMap<usize, Vec<usize>>,
    visited: &mut HashSet<usize>,
    dfs_visited: &mut HashSet<usize>,
) -> bool {
    visited.insert(src);
    dfs_visited.insert(src);
    match adj.get(&src) {
        Some(neighbors) => {
            for &nei in neighbors.iter() {
                if !visited.contains(&nei) {
                    if detect_cycle(nei, adj, visited, dfs_visited) {
                        return true;
                    }
                } else {
                    if dfs_visited.contains(&nei) {
                        return true;
                    }
                }
            }
        }
        None => {}
    }

    dfs_visited.remove(&src);
    false
}

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut visited = HashSet::new();
    let mut dfs_visited = HashSet::new();
    let mut adj = HashMap::new();
    for edge in prerequisites {
        let src = edge[0];
        let dest = edge[1];
        adj.entry(src as usize)
            .and_modify(|v: &mut Vec<usize>| v.push(dest as usize))
            .or_insert(vec![dest as usize]);
    }
    for i in 0..num_courses as usize {
        if !visited.contains(&i) {
            if detect_cycle(i, &adj, &mut visited, &mut dfs_visited) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::can_finish;
    #[test]
    fn run_tc1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        assert_eq!(can_finish(num_courses, prerequisites), true);
    }
    #[test]
    fn run_tc2() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(can_finish(num_courses, prerequisites), false);
    }
}

fn main() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    assert_eq!(can_finish(num_courses, prerequisites), true);
}
