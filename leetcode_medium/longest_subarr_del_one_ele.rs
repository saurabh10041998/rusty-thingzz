fn longest_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut left = 0;
    let mut zeros = 0;
    for right in 0..n {
        if nums[right] == 0 {
            zeros += 1;
        }
        while zeros > 1 {
            if nums[left] == 0 {
                zeros -= 1;
            }
            left += 1;
        }
        assert!(zeros <= 1);
        ans = usize::max(ans, right - left + 1 - zeros);
    }
    if ans == n {
        (ans - 1) as i32
    } else {
        ans as i32
    }
}

#[cfg(test)]
pub mod tests {
    use crate::longest_subarray;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 1, 0, 1];
        assert_eq!(longest_subarray(nums), 3);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        assert_eq!(longest_subarray(nums), 5);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![1, 1, 1];
        assert_eq!(longest_subarray(nums), 2);
    }
}

fn main() {
    let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
    assert_eq!(longest_subarray(nums), 5);
}
