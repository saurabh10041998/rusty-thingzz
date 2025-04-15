use std::collections::VecDeque;

fn is_dag(n: i32, adj: &Vec<Vec<usize>>) -> bool {
    let mut q = VecDeque::new();
    let mut indegree = vec![0; n as usize];
    for i in 0..n as usize {
        for &v in adj[i].iter() {
            indegree[v] += 1;
        }
    }
    for i in 0..n as usize {
        if indegree[i] == 0 {
            q.push_back(i);
        }
    }
    let mut cnt = 0;
    while let Some(vertex) = q.pop_front() {
        cnt += 1;
        for &nei in adj[vertex].iter() {
            indegree[nei as usize] -= 1;
            if indegree[nei as usize] == 0 {
                q.push_back(nei as usize);
            }
        }
    }
    cnt == n
}

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut adj = vec![];
    adj.resize_with(num_courses as usize, || Vec::new());
    for edge in prerequisites {
        let u = edge[0];
        let v = edge[1];
        adj[u as usize].push(v as usize);
    }
    is_dag(num_courses, &adj)
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
