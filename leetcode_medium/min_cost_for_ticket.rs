use std::collections::HashSet;

fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let mut days = days;
    let hs = days.drain(..).collect::<HashSet<i32>>();
    let mut dp = vec![0; 366];
    dp[0] = 0;
    for i in 1..=365usize {
        if !hs.contains(&(i as i32)) {
            dp[i] = dp[i - 1];
        } else {
            let c1 = costs[0] + dp[i - 1];
            let c2 = costs[1]
                + match i.checked_sub(7) {
                    Some(idx) => dp[idx],
                    None => dp[0],
                };
            let c3 = costs[2]
                + match i.checked_sub(30) {
                    Some(idx) => dp[idx],
                    None => dp[0],
                };
            dp[i] = i32::min(c1, i32::min(c2, c3));
        }
    }
    dp[365]
}

#[cfg(test)]
pub mod tests {
    use crate::mincost_tickets;
    #[test]
    fn run_tc1() {
        let days = vec![1, 4, 6, 7, 8, 20];
        let costs = vec![2, 7, 15];
        assert_eq!(mincost_tickets(days, costs), 11);
    }
    #[test]
    fn run_tc2() {
        let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
        let costs = vec![2, 7, 15];
        assert_eq!(mincost_tickets(days, costs), 17);
    }
}

fn main() {
    let days = vec![1, 4, 6, 7, 8, 20];
    let costs = vec![2, 7, 15];
    assert_eq!(mincost_tickets(days, costs), 11);
}
