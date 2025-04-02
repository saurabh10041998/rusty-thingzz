fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut lmax = vec![0i64; n];
    let mut rmax = vec![0i64; n];

    lmax[0] = 0;
    rmax[n - 1] = 0;

    for i in 0..=n - 2 {
        lmax[i + 1] = i64::max(lmax[i], nums[i] as i64);
        rmax[n - 2 - i] = i64::max(rmax[n - 1 - i], nums[n - 1 - i] as i64);
    }

    let mut ans = 0;
    for i in 0..n {
        ans = i64::max(ans, (lmax[i] - nums[i] as i64) * rmax[i]);
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::maximum_triplet_value;

    #[test]
    fn run_tc1() {
        let nums = vec![12, 6, 1, 2, 7];
        assert_eq!(maximum_triplet_value(nums), 77);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![1, 10, 3, 4, 19];
        assert_eq!(maximum_triplet_value(nums), 133);
    }

    #[test]
    fn run_tc3() {
        let nums = vec![1, 2, 3];
        assert_eq!(maximum_triplet_value(nums), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
