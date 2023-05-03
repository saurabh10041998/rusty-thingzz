use std::collections::HashSet;

fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let hs1 = nums1.into_iter().collect::<HashSet<i32>>();
    let hs2 = nums2.into_iter().collect::<HashSet<i32>>();

    let mut set1 = vec![];
    for x in hs1.iter() {
        if !hs2.contains(x) {
            set1.push(*x);
        }
    }

    let mut set2 = vec![];
    for x in hs2.iter() {
        if !hs1.contains(x) {
            set2.push(*x);
        }
    }
    vec![set1, set2]
}

#[cfg(test)]
pub mod tests {
    use crate::find_difference;
    #[test]
    fn run_tc1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        assert_eq!(find_difference(nums1, nums2), vec![vec![1, 3], vec![6, 4]]);
    }
    #[test]
    fn run_tc2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        assert_eq!(find_difference(nums1, nums2), vec![vec![3], vec![]]);
    }
}

fn main() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];
    assert_eq!(find_difference(nums1, nums2), vec![vec![1, 3], vec![6, 4]]);
}
