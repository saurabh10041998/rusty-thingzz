use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn bfs(graph: &mut HashMap<i32, Vec<usize>>, arr: &Vec<i32>, n: usize) -> i32 {
    let mut q = VecDeque::new();
    let mut level = 0;
    q.push_back(0usize);
    let mut visited = HashSet::new();
    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let curr = q.pop_front().unwrap();
            if curr == n - 1 {
                return level;
            }
            if visited.contains(&curr) || curr >= n {
                continue;
            }
            match curr.checked_sub(1) {
                Some(val) if !visited.contains(&val) => {
                    q.push_back(val);
                }
                Some(_) => {}
                None => {}
            };

            match curr.checked_add(1) {
                Some(val) if !visited.contains(&val) => {
                    if val < n {
                        q.push_back(val);
                    }
                }
                Some(_) => {}
                None => {}
            };

            match graph.get(&arr[curr]) {
                Some(neighbors) => {
                    for n in neighbors {
                        if !visited.contains(n) {
                            q.push_back(*n);
                        }
                    }
                    // To reduce MLE and minimize
                    // strain on memory
                    graph.remove(&arr[curr]);
                }
                None => {}
            }
            visited.insert(curr);
        }
        level += 1;
    }
    -1
}

fn min_jumps(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut graph = HashMap::new();
    for i in 0..n {
        graph
            .entry(arr[i])
            .and_modify(|v: &mut Vec<usize>| v.push(i))
            .or_insert(Vec::from([i]));
    }
    bfs(&mut graph, &arr, n)
}

#[cfg(test)]
pub mod tests {
    use crate::min_jumps;
    #[test]
    fn run_tc1() {
        let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        assert_eq!(min_jumps(arr), 3);
    }
    #[test]
    fn run_tc2() {
        let arr = vec![7];
        assert_eq!(min_jumps(arr), 0);
    }

    #[test]
    fn run_tc3() {
        let arr = vec![7, 6, 9, 6, 9, 6, 9, 7];
        assert_eq!(min_jumps(arr), 1);
    }
}

fn main() {
    let arr = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
    assert_eq!(min_jumps(arr), 3);
}
