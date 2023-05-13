use std::collections::HashMap;

const MOD: i64 = i64::pow(10, 9) + 7;
fn dfs(low: i32, high: i32, zero: i32, one: i32, len: usize, dp: &mut HashMap<usize, i64>) -> i64 {
    if len > high as usize {
        return 0;
    }
    match dp.get(&len) {
        Some(ans) => return *ans,
        None => {}
    }
    let mut ans = if len >= low as usize { 1 } else { 0 };
    ans = (ans
        + dfs(low, high, zero, one, len + zero as usize, dp)        // add 0 zero times
        + dfs(low, high, zero, one, len + one as usize, dp))        // add 1 one times
        % MOD;
    dp.entry(len).or_insert(ans);
    ans
}

fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    let mut dp = HashMap::new();
    (dfs(low, high, zero, one, 0, &mut dp) % MOD) as i32
}

#[cfg(test)]
pub mod tests {
    use crate::count_good_strings;
    #[test]
    fn run_tc1() {
        let low = 3;
        let high = 3;
        let zero = 1;
        let one = 1;
        assert_eq!(count_good_strings(low, high, zero, one), 8);
    }
    #[test]
    fn run_tc2() {
        let low = 2;
        let high = 3;
        let zero = 1;
        let one = 2;
        assert_eq!(count_good_strings(low, high, zero, one), 5);
    }
}

fn main() {
    let low = 2;
    let high = 3;
    let zero = 1;
    let one = 2;
    assert_eq!(count_good_strings(low, high, zero, one), 8);
}
