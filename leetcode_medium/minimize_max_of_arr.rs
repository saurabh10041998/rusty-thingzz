fn ceil(a: i64, b: i64) -> i64 {
    a / b + (a % b != 0) as i64
}

fn minimize_array_value(nums: Vec<i32>) -> i32 {
    let mut prefix_sum: i64 = 0;
    nums.into_iter()
        .enumerate()
        .map(|(idx, val)| {
            prefix_sum += val as i64;
            ceil(prefix_sum, (idx + 1) as i64)
        })
        .max()
        .unwrap() as i32
}

#[cfg(test)]
pub mod tests {
    use crate::minimize_array_value;
    #[test]
    fn run_tc1() {
        let nums = vec![3, 7, 1, 6];
        assert_eq!(minimize_array_value(nums), 5);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![10, 1];
        assert_eq!(minimize_array_value(nums), 10);
    }
}

fn main() {
    let nums = vec![3, 7, 1, 6];
    assert_eq!(minimize_array_value(nums), 5);
}
