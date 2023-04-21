use std::collections::HashMap;

const MOD: i32 = i32::pow(10, 9) + 7;

fn helper(
    n: i32,
    min_profit: i32,
    group: &Vec<i32>,
    idx: usize,
    p: i32,
    profit: &Vec<i32>,
    dp: &mut HashMap<(i32, i32, i32), i32>,
) -> i32 {
    if idx == group.len() {
        if p >= min_profit {
            return 1;
        } else {
            return 0;
        }
    }
    match dp.get(&(n, idx as i32, p)) {
        Some(val) => {
            return *val;
        }
        None => {}
    }

    // Skip the group
    let ans = helper(n, min_profit, group, idx + 1, p, profit, dp);
    dp.entry((n, idx as i32, p)).or_insert(ans);

    // Include the group
    if n - group[idx] >= 0 {
        let ans = helper(
            n - group[idx],
            min_profit,
            group,
            idx + 1,
            p + profit[idx],
            profit,
            dp,
        );
        dp.entry((n, idx as i32, p))
            .and_modify(|ways| *ways = (*ways % MOD + ans % MOD) % MOD);
    }

    // Return the ans
    match dp.get(&(n, idx as i32, p)) {
        Some(val) => *val,
        None => unreachable!(),
    }
}

fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();
    helper(n, min_profit, &group, 0, 0, &profit, &mut dp)
}

#[cfg(test)]
pub mod tests {
    use crate::profitable_schemes;
    #[test]
    fn run_tc1() {
        let n = 5;
        let min_profit = 3;
        let group = vec![2, 2];
        let profit = vec![2, 3];
        assert_eq!(profitable_schemes(n, min_profit, group, profit), 2);
    }
    #[test]
    fn run_tc2() {
        let n = 10;
        let min_profit = 5;
        let group = vec![2, 3, 5];
        let profit = vec![6, 7, 8];
        assert_eq!(profitable_schemes(n, min_profit, group, profit), 7);
    }
    #[test]
    fn run_tc3() {
        let n = 100;
        let min_profit = 100;
        let group = vec![
            2, 5, 36, 2, 5, 5, 14, 1, 12, 1, 14, 15, 1, 1, 27, 13, 6, 59, 6, 1, 7, 1, 2, 7, 6, 1,
            6, 1, 3, 1, 2, 11, 3, 39, 21, 20, 1, 27, 26, 22, 11, 17, 3, 2, 4, 5, 6, 18, 4, 14, 1,
            1, 1, 3, 12, 9, 7, 3, 16, 5, 1, 19, 4, 8, 6, 3, 2, 7, 3, 5, 12, 6, 15, 2, 11, 12, 12,
            21, 5, 1, 13, 2, 29, 38, 10, 17, 1, 14, 1, 62, 7, 1, 14, 6, 4, 16, 6, 4, 32, 48,
        ];
        let profit = vec![
            21, 4, 9, 12, 5, 8, 8, 5, 14, 18, 43, 24, 3, 0, 20, 9, 0, 24, 4, 0, 0, 7, 3, 13, 6, 5,
            19, 6, 3, 14, 9, 5, 5, 6, 4, 7, 20, 2, 13, 0, 1, 19, 4, 0, 11, 9, 6, 15, 15, 7, 1, 25,
            17, 4, 4, 3, 43, 46, 82, 15, 12, 4, 1, 8, 24, 3, 15, 3, 6, 3, 0, 8, 10, 8, 10, 1, 21,
            13, 10, 28, 11, 27, 17, 1, 13, 10, 11, 4, 36, 26, 4, 2, 2, 2, 10, 0, 11, 5, 22, 6,
        ];
        assert_eq!(profitable_schemes(n, min_profit, group, profit), 692206787);
    }
}

fn main() {
    let n = 10;
    let min_profit = 5;
    let group = vec![2, 3, 5];
    let profit = vec![6, 7, 8];
    assert_eq!(profitable_schemes(n, min_profit, group, profit), 7);
}
