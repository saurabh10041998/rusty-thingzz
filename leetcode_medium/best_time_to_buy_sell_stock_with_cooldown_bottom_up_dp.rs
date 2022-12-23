use std::collections::HashMap;

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();

    let n = prices.len();
    for i in (0..n).rev() {
        for buy in [false, true] {
            if buy {
                let further_1: (i32, i32) = (
                    dp.get(&(i + 1, false)).cloned().unwrap_or(0),
                    dp.get(&(i + 1, true)).cloned().unwrap_or(0),
                );
            let ans = i32::max(-prices[i] + further_1.0, 0  + further_1.1);
            dp.insert((i, buy), ans);
                
            } else {
                let further_1: (i32, i32) = (
                    dp.get(&(i + 1, false)).cloned().unwrap_or(0),
                    dp.get(&(i + 1, true)).cloned().unwrap_or(0)
                );
                // this is required due to cooldown restriction
                let further_2:(i32, i32) = (
                    dp.get(&(i + 2, false)).cloned().unwrap_or(0),
                    dp.get(&(i + 2, true)).cloned().unwrap_or(0)
                );

                let ans = i32::max(prices[i] + further_2.1, 0 + further_1.0);
                dp.insert((i, buy), ans);
            }
        }
    }
    *dp.get(&(0, true)).unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::max_profit;
    #[test]
    fn run_tc1() {
        let prices = vec![1, 2, 3, 0, 2];
        assert_eq!(max_profit(prices), 3);
    }

    #[test]
    fn run_tc2() {
        let prices = vec![1];
        assert_eq!(max_profit(prices), 0);
    }
}

fn main() {
    let prices = vec![1, 2, 3, 0, 2];
    assert_eq!(max_profit(prices), 3);
}
