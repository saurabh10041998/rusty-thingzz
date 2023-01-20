use std::collections::HashSet;
fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = HashSet::new();
    let n = nums.len();
    for bit in 0..usize::pow(2, n as u32) {
        let mut test = vec![];
        for i in 0..n {
            if (bit >> i) & 1 != 0 {
                test.push(nums[i])
            }
        }
        if test.len() >= 2 {
            let mut is_increasing = true;
            for j in 0..test.len() - 1 {
                is_increasing &= test[j] <= test[j + 1];
            }
            if is_increasing {
                ans.insert(test);
            }
        }
    }
    ans.drain().collect()
}

#[cfg(test)]
pub mod tests {
    use crate::find_subsequences;

    #[test]
    fn run_tc1() {
        let nums = vec![4, 6, 7, 7];
        assert_eq!(
            find_subsequences(nums),
            vec![
                vec![4, 6],
                vec![4, 6, 7],
                vec![4, 6, 7, 7],
                vec![4, 7],
                vec![4, 7, 7],
                vec![6, 7],
                vec![6, 7, 7],
                vec![7, 7]
            ]
        )
    }
    #[test]
    fn run_tc2() {
        let nums = vec![4, 4, 3, 2, 1];
        assert_eq!(find_subsequences(nums), vec![vec![4, 4]]);
    }
}

fn main() {
    let nums = vec![4, 4, 3, 2, 1];
    assert_eq!(find_subsequences(nums), vec![vec![4, 4]])
}
