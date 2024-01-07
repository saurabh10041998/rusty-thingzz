use std::collections::HashMap;

fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![];
    dp.resize_with(n, || HashMap::new());
    let mut total_seq = 0;
    for idx in 0..n {
        for j in 0..idx {
            let diff = nums[idx] as i64 - nums[j] as i64;
            dp[idx]
                .entry(diff)
                .and_modify(|f: &mut i32| *f += 1)
                .or_insert(1);
            match dp[j].get(&diff) {
                Some(&count) => {
                    dp[idx].entry(diff).and_modify(|f| *f += count);
                    total_seq += count;
                }
                None => {}
            }
        }
    }
    total_seq
}

#[cfg(test)]
pub mod tests {
    use crate::number_of_arithmetic_slices;
    #[test]
    fn run_tc1() {
        let nums = vec![2, 4, 6, 8, 10];
        assert_eq!(number_of_arithmetic_slices(nums), 7);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![7, 7, 7, 7, 7];
        assert_eq!(number_of_arithmetic_slices(nums), 16);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![0, 2000000000, -294967296];
        assert_eq!(number_of_arithmetic_slices(nums), 0);
    }
}

fn main() {
    let nums = vec![7, 7, 7, 7, 7];
    assert_eq!(number_of_arithmetic_slices(nums), 16);
}
