use std::collections::HashMap;

fn solve(
    idx: usize,
    mask: usize,
    k: usize,
    temp: &[i32],
    dp: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if idx == k {
        if mask == 0 {
            let ans = 0;
            dp.entry((idx, mask)).or_insert(ans);
            return ans;
        } else {
            let ans = i32::MAX;
            dp.entry((idx, mask)).or_insert(ans);
            return ans;
        }
    }
    if let Some(ans) = dp.get(&(idx, mask)) {
        return *ans;
    }
    let mut highest = i32::MAX;
    let mut current = mask;
    while current > 0 {
        let maxi = i32::max(solve(idx + 1, mask & !current, k, temp, dp), temp[current]);
        highest = i32::min(highest, maxi);
        current = (current - 1) & mask;
    }
    dp.entry((idx, mask)).or_insert(highest);
    highest
}

fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
    let n = cookies.len();
    let mut dp = HashMap::new();
    let mut temp = vec![0; 1 << n];
    for (mask, item) in temp.iter_mut().enumerate().take(1 << n) {
        let mut current = 0;
        for (j, _) in cookies.iter().enumerate().take(n) {
            if (1 << j) & mask > 0 {
                current += cookies[j]
            }
        }
        *item = current;
    }
    solve(0, (1 << n) - 1, k as usize, &temp, &mut dp)
}

#[cfg(test)]
pub mod tests {
    use crate::distribute_cookies;
    #[test]
    fn run_tc1() {
        let cookies = vec![8, 15, 10, 20, 8];
        let k = 2;
        assert_eq!(distribute_cookies(cookies, k), 31);
    }
    #[test]
    fn run_tc2() {
        let cookies = vec![6, 1, 3, 2, 2, 4, 1, 2];
        let k = 3;
        assert_eq!(distribute_cookies(cookies, k), 7);
    }
}

fn main() {
    let cookies = vec![6, 1, 3, 2, 2, 4, 1, 2];
    let k = 3;
    assert_eq!(distribute_cookies(cookies, k), 7);
}
