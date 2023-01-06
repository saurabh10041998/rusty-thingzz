use std::collections::BTreeSet;
use std::ops::Bound::*;
fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    let mut costs = costs;
    costs.sort();
    let cost_log_set = costs
        .iter()
        .scan(0i64, |sum, i| {
            *sum += *i as i64;
            Some(*sum as i64)
        })
        .collect::<BTreeSet<i64>>();
    let array = cost_log_set
        .range((Unbounded, Included(coins as i64)))
        .collect::<Vec<_>>();
    array.len() as i32
}

#[cfg(test)]
pub mod tests {
    use crate::max_ice_cream;
    #[test]
    fn run_tc1() {
        let costs = vec![1, 3, 2, 4, 1];
        let coins = 7;
        assert_eq!(max_ice_cream(costs, coins), 4);
    }
    #[test]
    fn run_tc2() {
        let costs = vec![10, 6, 8, 7, 7, 8];
        let coins = 5;
        assert_eq!(max_ice_cream(costs, coins), 0);
    }
    #[test]
    fn run_tc3() {
        let costs = vec![1, 6, 3, 1, 2, 5];
        let coins = 20;
        assert_eq!(max_ice_cream(costs, coins), 6);
    }
}

fn main() {
    let costs = vec![1, 3, 2, 4, 1];
    let coins = 7;
    assert_eq!(max_ice_cream(costs, coins), 4);
}
