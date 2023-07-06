fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut left = 0;
    let mut ans = usize::MAX;
    for (right, &num) in nums.iter().enumerate() {
        sum += num;
        while sum >= target {
            ans = usize::min(ans, right - left + 1);
            sum -= nums[left];
            left += 1;
        }
    }
    match ans {
        usize::MAX => 0,
        _ => ans as i32
    }
}

#[cfg(test)]
pub mod tests {
    use crate::min_sub_array_len;
    #[test]
    fn run_tc1() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        assert_eq!(min_sub_array_len(target, nums), 2);
    }
    #[test]
    fn run_tc2() {
        let target = 4;
        let nums = vec![1, 4, 4];
        assert_eq!(min_sub_array_len(target, nums), 1);
    }
    #[test]
    fn run_tc3() {
        let target = 11;
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(min_sub_array_len(target, nums), 0);
    }
}

fn main() {
    let target = 11;
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
    assert_eq!(min_sub_array_len(target, nums), 0);
}
