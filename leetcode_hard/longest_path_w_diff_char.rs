use std::collections::HashSet;

fn longest_path_helper(adj: &Vec<HashSet<usize>>, src: usize, s: &Vec<char>, ans: &mut i32) -> i32 {
    let mut max = 0;
    let mut second_max = 0;
    for &v in adj[src].iter() {
        let res = longest_path_helper(adj, v, s, ans);
        if s[v] == s[src] {
            continue;
        }
        if res > max {
            second_max = max;
            max = res;
        } else if res > second_max {
            second_max = res;
        }
        *ans = i32::max(*ans, 1 + max + second_max);
    }
    1 + max
}

fn longest_path(parent: Vec<i32>, s: String) -> i32 {
    let n = parent.len();
    let mut adj = vec![HashSet::new(); n];
    for (i, &p) in parent.iter().enumerate() {
        if p == -1 {
            continue;
        }
        adj[p as usize].insert(i);
    }
    let s = s.chars().collect::<Vec<char>>();
    let mut ans = 1; // answer will always be one as we have node 0
    longest_path_helper(&adj, 0, &s, &mut ans);
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::longest_path;
    #[test]
    fn run_tc1() {
        let parent = vec![-1, 0, 0, 1, 1, 2];
        let s = String::from("abacbe");
        assert_eq!(longest_path(parent, s), 3);
    }
    #[test]
    fn run_tc2() {
        let parent = vec![-1, 0, 0, 0];
        let s = String::from("aabc");
        assert_eq!(longest_path(parent, s), 3);
    }
}

fn main() {
    let parent = vec![-1, 0, 0, 1, 1, 2];
    let s = String::from("abacbe");
    assert_eq!(longest_path(parent, s), 3);
}
