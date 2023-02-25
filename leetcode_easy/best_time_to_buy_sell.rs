fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, 1);
    let mut max_profit = 0;
    while r < prices.len() {
        let profit = prices[r] - prices[l];
        if prices[l] < prices[r] {
            max_profit = i32::max(max_profit, profit);
        } else {
            l = r;
        }
        r += 1;
    }
    max_profit
}

#[cfg(test)]
pub mod tests {
    use crate::max_profit;
    #[test]
    fn run_tc1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }
    #[test]
    fn run_tc2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(max_profit(prices), 5);
}
