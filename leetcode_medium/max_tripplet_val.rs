fn maximum_tripplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut max_diff = 0;
    let mut max_tripplet = 0;
    let mut lmax = 0i64;
    for i in 0..n - 1 {
        max_diff = i64::max(max_diff, lmax - nums[i] as i64);
        lmax = i64::max(lmax, nums[i] as i64);
        max_tripplet = i64::max(max_tripplet, max_diff * nums[i + 1] as i64);
    }
    max_tripplet
}

#[cfg(test)]
mod tests {
    use crate::maximum_tripplet_value;
    #[test]
    fn run_tc1() {
        let nums = vec![12, 6, 1, 2, 7];
        assert_eq!(maximum_tripplet_value(nums), 77);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 10, 3, 4, 19];
        assert_eq!(maximum_tripplet_value(nums), 133);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![1, 2, 3];
        assert_eq!(maximum_tripplet_value(nums), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
