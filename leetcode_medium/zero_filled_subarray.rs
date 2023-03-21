fn sum_up_to(c: i64) -> i64 {
    (c * (c + 1)) / 2
}

fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut res: i64 = 0;
    let mut c = 0;
    for (_, v) in nums.into_iter().enumerate() {
        if v != 0 {
            res += sum_up_to(c as i64);
            c = 0;
        } else {
            c += 1;
        }
    }
    res += if c == 0 { 0 } else { sum_up_to(c) };
    res
}
#[cfg(test)]
pub mod tests {
    use crate::zero_filled_subarray;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];
        assert_eq!(zero_filled_subarray(nums), 6);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![0, 0, 0, 2, 0, 0];
        assert_eq!(zero_filled_subarray(nums), 9);
    }
}
fn main() {
    let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];
    assert_eq!(zero_filled_subarray(nums), 6);
}
