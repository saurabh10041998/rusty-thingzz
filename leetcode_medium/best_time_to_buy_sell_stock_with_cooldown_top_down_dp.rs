use std::collections::HashMap;
fn inner_f(
    idx: usize,
    right_to_buy: bool,
    prices: &Vec<i32>,
    dp: &mut HashMap<(usize, bool), i32>,
) -> i32 {
    if idx >= prices.len() {
        return 0;
    }
    match dp.get(&(idx, right_to_buy)) {
        Some(val) => {
            return *val;
        }
        None => {}
    }
    let ans = if right_to_buy {
        i32::max(
            -prices[idx] + inner_f(idx + 1, false, prices, dp),
            0 + inner_f(idx + 1, true, prices, dp),
        )
    } else {
        i32::max(
            prices[idx] + inner_f(idx + 2, true, prices, dp),
            0 + inner_f(idx + 1, false, prices, dp),
        )
    };
    dp.insert((idx, right_to_buy), ans);
    ans
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();
    inner_f(0, /*right_to_buy */ true, &prices, &mut dp);
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
