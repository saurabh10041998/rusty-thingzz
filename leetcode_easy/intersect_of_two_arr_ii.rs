use std::collections::HashMap;

fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut hm = HashMap::new();
    for &k in nums1.iter() {
        hm.entry(k).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut ans = vec![];
    for &k in nums2.iter() {
        match hm.get(&k) {
            Some(&v) => {
                if v > 0 {
                    ans.push(k);
                    hm.entry(k).and_modify(|x| *x -= 1);
                }
            }
            None => {}
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::intersect;
    #[test]
    fn run_tc1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersect(nums1, nums2), vec![2, 2]);
    }
    #[test]
    fn run_tc2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(intersect(nums1, nums2), vec![9, 4]);
    }
}

fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    assert_eq!(intersect(nums1, nums2), vec![2, 2]);
}
