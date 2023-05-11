use std::mem;

fn lcs(num1: &Vec<i32>, num2: &Vec<i32>) -> i32 {
    let (n, m) = (num1.len(), num2.len());
    let mut prev = vec![0; (m + 1) as usize];
    let mut curr = vec![0; (m + 1) as usize];
    for i in 1..=n {
        for j in 1..=m {
            if num1[i - 1] == num2[j - 1] {
                curr[j] = 1 + prev[j - 1];
            } else {
                curr[j] = i32::max(prev[j], curr[j - 1]);
            }
        }
        prev = mem::replace(&mut curr, vec![0; (m + 1) as usize]);
    }
    prev[m]
}

fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    lcs(&nums1, &nums2)
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
