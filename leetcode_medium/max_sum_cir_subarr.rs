// Answer to this problem is combination of
// max sum subarray problem  and max wrap around sum (prefix[i] + suffix[i + 1]) ??
fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    // PART 1: Solve for max wrap around sum.. 
    // maximum wrap around sum
    // max(prefix_sum uptil i  +  max suffix sum till i + 1)
    let n = nums.len();
    let mut max_suffix = vec![0; n];
    let mut curr_sum = nums[n - 1];
    max_suffix[n - 1] = nums[n - 1];
    for i in (0..n - 1).rev() {
        curr_sum += nums[i];
        max_suffix[i] = i32::max(max_suffix[i + 1], curr_sum);
    }
    let mut max_prefix = vec![0; n];
    let mut curr_sum = nums[0];
    max_prefix[0] = nums[0];
    for i in 1..n {
        curr_sum += nums[i];
        max_prefix[i] = i32::max(max_prefix[i - 1], curr_sum);
    }

    let mut wrap_around_sum = i32::MIN;
    for i in 0..n - 1 {
        wrap_around_sum = i32::max(wrap_around_sum, max_prefix[i] + max_suffix[i + 1]);
    }

    // PART - 2 : Maximum sum subarray
    // Kadane's algo for max_sum_subarray
    let mut max_sum = i32::MIN;
    let mut max_end = 0;
    for i in 0..n {
        max_end += nums[i];
        max_sum = i32::max(max_sum, max_end);
        if max_end < 0 {
            max_end = 0;
        }
    }
    i32::max(max_sum, wrap_around_sum)
}

#[cfg(test)]
pub mod tests {
    use crate::max_subarray_sum_circular;
    #[test]
    fn run_tc1() {
        let nums = vec![1, -2, 3, -2];
        assert_eq!(max_subarray_sum_circular(nums), 3);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![5, -3, 5];
        assert_eq!(max_subarray_sum_circular(nums), 10);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![-3, -2, -3];
        assert_eq!(max_subarray_sum_circular(nums), -2);
    }
}
fn main() {
    let nums = vec![1, -2, 3, -2];
    assert_eq!(max_subarray_sum_circular(nums), 3);
}
