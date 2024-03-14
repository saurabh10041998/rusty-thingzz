fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    // number of subarrays with sum <= x
    let helper = |x| {
        if x < 0 {
            return 0;
        }
        let (mut l, mut res) = (0, 0);
        let mut sum = 0;
        for r in 0..nums.len() {
            sum += nums[r];
            while sum > x {
                sum -= nums[l];
                l += 1;
            }
            res += r - l + 1;
        }
        res
    };
    (helper(goal) - helper(goal - 1)) as i32
}

#[cfg(test)]
pub mod tests {
    use crate::num_subarrays_with_sum;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 0, 1, 0, 1];
        let goal = 2;
        assert_eq!(num_subarrays_with_sum(nums, goal), 4);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![0, 0, 0, 0, 0];
        let goal = 0;
        assert_eq!(num_subarrays_with_sum(nums, goal), 15);
    }
}

fn main() {
    let nums = vec![1, 0, 1, 0, 1];
    let goal = 2;
    assert_eq!(num_subarrays_with_sum(nums, goal), 4);
}
