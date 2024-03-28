use std::collections::HashMap;

fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut hs = HashMap::new();
    let (mut i, mut ans) = (0, 0);
    for j in 0..nums.len() {
        hs.entry(nums[j])
            .and_modify(|f: &mut usize| *f += 1)
            .or_insert(1);
        while *hs.get(&nums[j]).unwrap() > k as usize {
            hs.entry(nums[i]).and_modify(|f: &mut usize| *f -= 1);
            i += 1;
        }
        ans = usize::max(ans, j - i + 1);
    }
    ans as i32
}

#[cfg(test)]
pub mod tests {
    use crate::max_subarray_length;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
        let k = 2;
        assert_eq!(max_subarray_length(nums, k), 6);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2];
        let k = 1;
        assert_eq!(max_subarray_length(nums, k), 2);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![5, 5, 5, 5, 5, 5, 5];
        let k = 4;
        assert_eq!(max_subarray_length(nums, k), 4);
    }
}

fn main() {
    let nums = vec![5, 5, 5, 5, 5, 5, 5];
    let k = 4;
    assert_eq!(max_subarray_length(nums, k), 4);
}
