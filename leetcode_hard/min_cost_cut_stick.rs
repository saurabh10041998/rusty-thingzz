use std::collections::HashMap;
fn dfs(cuts: &Vec<i32>, dp: &mut HashMap<(usize, usize), i32>, l: usize, r: usize) -> i32 {
    if r - l == 1 {
        return 0;
    }
    match dp.get(&(l, r)) {
        Some(ans) => return *ans,
        None => {}
    };
    let mut res = i32::MAX;
    for c in cuts {
        let c = *c as usize;
        if c > l && c < r {
            res = i32::min(
                res,
                (r - l) as i32 + dfs(cuts, dp, l, c) + dfs(cuts, dp, c, r),
            );
        }
    }
    let res = if res == i32::MAX { 0 } else { res };
    dp.entry((l, r)).or_insert(res);
    res
}

fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();
    dfs(&cuts, &mut dp, 0, n as usize)
}

#[cfg(test)]
pub mod tests {
    use crate::min_cost;
    #[test]
    fn run_tc1() {
        let n = 7;
        let cuts = vec![1, 3, 4, 5];
        assert_eq!(min_cost(n, cuts), 16);
    }
    #[test]
    fn run_tc2() {
        let n = 9;
        let cuts = vec![5, 6, 1, 4, 2];
        assert_eq!(min_cost(n, cuts), 22);
    }
}

fn main() {
    let n = 9;
    let cuts = vec![5, 6, 1, 4, 2];
    assert_eq!(min_cost(n, cuts), 16);
}
