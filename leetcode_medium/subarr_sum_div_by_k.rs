use std::collections::HashMap;

fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut hm = HashMap::new();
    let mut curr_sum = 0;
    hm.entry(0).or_insert(1);
    let mut cnt = 0;
    for n in nums {
        curr_sum = (curr_sum + n + k) % k;
        if curr_sum < 0 {
            curr_sum += k;
        }
        if hm.contains_key(&curr_sum) {
            cnt += *hm.get(&curr_sum).unwrap();
        }
        hm.entry(curr_sum).and_modify(|v| *v += 1).or_insert(1);
    }
    cnt
}

#[cfg(test)]
pub mod tests {
    use crate::subarrays_div_by_k;
    #[test]
    fn run_tc1() {
        let nums = vec![4, 5, 0, -2, -3, 1];
        let k = 5;
        assert_eq!(subarrays_div_by_k(nums, k), 7);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![5];
        let k = 9;
        assert_eq!(subarrays_div_by_k(nums, k), 0);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![7,4,-10];
        let k = 5;
        assert_eq!(subarrays_div_by_k(nums, k), 1);
    }
}

fn main() {
    let nums = vec![4, 5, 0, -2, -3, 1];
    let k = 5;
    assert_eq!(subarrays_div_by_k(nums, k), 7);
}
