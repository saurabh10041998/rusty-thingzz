use std::collections::HashMap;

fn lcs(
    num1: &Vec<i32>,
    num2: &Vec<i32>,
    i: usize,
    j: usize,
    dp: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if i == 0 || j == 0 {
        return 0;
    }
    match dp.get(&(i, j)) {
        Some(ans) => return *ans,
        None => {}
    };
    let ans = if num1[i - 1] == num2[j - 1] {
        1 + lcs(num1, num2, i - 1, j - 1, dp)
    } else {
        i32::max(lcs(num1, num2, i - 1, j, dp), lcs(num1, num2, i, j - 1, dp))
    };

    dp.entry((i, j)).or_insert(ans);
    ans
}

fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();
    let (n, m) = (nums1.len(), nums2.len());
    lcs(&nums1, &nums2, n, m, &mut dp)
}

#[cfg(test)]
pub mod tests {
    use crate::max_uncrossed_lines;
    #[test]
    fn run_tc1() {
        let nums1 = vec![1, 4, 2];
        let nums2 = vec![1, 2, 4];
        assert_eq!(max_uncrossed_lines(nums1, nums2), 2);
    }
    #[test]
    fn run_tc2() {
        let nums1 = vec![2, 5, 1, 2, 5];
        let nums2 = vec![10, 5, 2, 1, 5, 2];
        assert_eq!(max_uncrossed_lines(nums1, nums2), 3);
    }
    #[test]
    fn run_tc3() {
        let nums1 = vec![1, 3, 7, 1, 7, 5];
        let nums2 = vec![1, 9, 2, 5, 1];
        assert_eq!(max_uncrossed_lines(nums1, nums2), 2);
    }
}

fn main() {
    let nums1 = vec![1, 3, 7, 1, 7, 5];
    let nums2 = vec![1, 9, 2, 5, 1];
    assert_eq!(max_uncrossed_lines(nums1, nums2), 2);
}
