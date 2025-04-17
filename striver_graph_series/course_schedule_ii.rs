use std::collections::{HashMap, VecDeque};

fn topo_sort(num_courses: usize, adj: &HashMap<usize, Vec<usize>>) -> Vec<i32> {
    let mut indegree = vec![0; num_courses];

    for i in 0..num_courses {
        match adj.get(&i) {
            Some(neigh) => {
                for &dst in neigh.iter() {
                    indegree[dst] += 1;
                }
            }
            None => {}
        }
    }

    let mut q = VecDeque::new();
    for i in 0..num_courses {
        if indegree[i] == 0 {
            q.push_back(i);
        }
    }

    let mut ans = vec![];

    while let Some(vertex) = q.pop_front() {
        ans.push(vertex as i32);
        match adj.get(&vertex) {
            Some(neighbors) => {
                for &nei in neighbors {
                    indegree[nei] -= 1;
                    if indegree[nei] == 0 {
                        q.push_back(nei);
                    }
                }
            }
            None => {}
        }
    }

    if ans.len() == num_courses {
        ans
    } else {
        vec![]
    }
}

fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut adj = HashMap::new();

    for edge in prerequisites {
        let depedency = edge[1] as usize;
        let dependent = edge[0] as usize;
        adj.entry(depedency)
            .and_modify(|v: &mut Vec<usize>| v.push(dependent))
            .or_insert(vec![dependent]);
    }

    topo_sort(num_courses as usize, &adj)
}

#[cfg(test)]
mod tests {
    use crate::find_order;
    #[test]
    fn run_tc1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        assert_eq!(find_order(num_courses, prerequisites), vec![0, 1]);
    }
    #[test]
    fn run_tc2() {
        let num_courses = 4;
        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        assert_eq!(find_order(num_courses, prerequisites), vec![0, 1, 2, 3]);
    }
}

fn main() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    assert_eq!(find_order(num_courses, prerequisites), vec![0, 1]);
}
