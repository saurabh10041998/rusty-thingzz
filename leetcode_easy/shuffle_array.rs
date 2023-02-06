fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut ans = vec![];
    let _n = 2 * n;
    let mut i = 0;
    let mut j = n as usize;
    let mut first = true;
    while j < _n as usize {
        match first {
            true => {
                ans.push(nums[i]);
                first = false;
                i += 1;
            }
            false => {
                ans.push(nums[j]);
                first = true;
                j += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::shuffle;
    #[test]
    fn run_tc1() {
        let nums = vec![2, 5, 1, 3, 4, 7];
        let n = 3;
        assert_eq!(shuffle(nums, n), vec![2, 3, 5, 4, 1, 7]);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 2, 3, 4, 4, 3, 2, 1];
        let n = 4;
        assert_eq!(shuffle(nums, n), vec![1, 4, 2, 3, 3, 2, 4, 1]);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![1, 1, 2, 2];
        let n = 2;
        assert_eq!(shuffle(nums, n), vec![1, 2, 1, 2]);
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 4, 3, 2, 1];
    let n = 4;
    assert_eq!(shuffle(nums, n), vec![1, 4, 2, 3, 3, 2, 4, 1]);
}
