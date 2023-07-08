fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let n = weights.len();
    let mut pair_sum = vec![0; n - 1];
    for (idx, w) in weights.windows(2).enumerate() {
        assert_eq!(w.len(), 2);
        pair_sum[idx] = (w[0] + w[1]) as i64;
    }
    pair_sum.sort();
    let min = pair_sum.iter().take(k as usize).sum::<i64>();
    let max = pair_sum.iter().rev().take(k as usize).sum::<i64>();
    max - min
}

#[cfg(test)]
pub mod tests {
    use crate::put_marbles;
    #[test]
    fn run_tc1() {
        let weights = vec![1, 3, 5, 1];
        let k = 2;
        assert_eq!(put_marbles(weights, k), 4);
    }
    #[test]
    fn run_tc2() {
        let weights = vec![1, 3];
        let k = 2;
        assert_eq!(put_marbles(weights, k), 0);
    }
}

fn main() {
    let weights = vec![1, 3];
    let k = 2;
    assert_eq!(put_marbles(weights, k), 0);
}
