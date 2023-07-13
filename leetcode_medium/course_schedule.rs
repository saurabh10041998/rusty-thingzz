use std::collections::VecDeque;

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = num_courses as usize;
    let mut graph = vec![vec![]; n];
    let mut indegree = vec![0; n];
    for subjects in prerequisites {
        let to = subjects[0];
        let from = subjects[1];
        graph[from as usize].push(to as usize);
        indegree[to as usize] += 1;
    }
    let mut q = VecDeque::new();
    let mut ans = vec![];
    for (idx, &v) in indegree.iter().enumerate() {
        if v == 0 {
            q.push_back(idx);
            ans.push(idx);
        }
    }
    while let Some(v) = q.pop_front() {
        for &neighbor in graph[v].iter() {
            indegree[neighbor] -= 1;
            if indegree[neighbor] == 0 {
                q.push_back(neighbor);
                ans.push(neighbor);
            }
        }
    }
    ans.len() == n
}

#[cfg(test)]
pub mod tests {
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
    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    assert!(!can_finish(num_courses, prerequisites));
}
