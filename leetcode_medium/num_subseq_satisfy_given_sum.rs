const MOD: i32 = i32::pow(10, 9) + 7;

fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let n = nums.len();
    let mut ans = 0;
    let mut powers = vec![0; n];
    powers[0] = 1;
    for i in 1..n {
        powers[i] = (powers[i - 1] % MOD * 2) % MOD;
    }
    let (mut l, mut r) = (0, n as i32 - 1);
    while l <= r {
        if nums[l as usize] + nums[r as usize] > target {
            r -= 1;
        } else {
            ans = (ans % MOD + powers[(r - l) as usize] % MOD) % MOD;
            l += 1;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::num_subseq;
    #[test]
    fn run_tc1() {
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        assert_eq!(num_subseq(nums, target), 4);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![3, 3, 6, 8];
        let target = 10;
        assert_eq!(num_subseq(nums, target), 6);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        assert_eq!(num_subseq(nums, target), 61);
    }
}
fn main() {
    let nums = vec![2, 3, 3, 4, 6, 7];
    let target = 12;
    assert_eq!(num_subseq(nums, target), 61);
}
