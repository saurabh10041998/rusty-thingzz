fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut product: i64 = 1;
    let k = k as i64;
    for j in 0..nums.len() {
        product *= nums[j] as i64;
        while product >= k && i <= j {
            product /= nums[i] as i64;
            i += 1;
        }
        if product <= k {
            ans += j - i + 1;
        }
    }
    ans as i32
}

#[cfg(test)]
pub mod tests {
    use crate::num_subarray_product_less_than_k;
    #[test]
    fn run_tc1() {
        let nums = vec![10, 5, 2, 6];
        let k = 100;
        assert_eq!(num_subarray_product_less_than_k(nums, k), 8);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 2, 3];
        let k = 0;
        assert_eq!(num_subarray_product_less_than_k(nums, k), 0);
    }
}

fn main() {
    let nums = vec![10, 5, 2, 6];
    let k = 100;
    assert_eq!(num_subarray_product_less_than_k(nums, k), 8);
}
