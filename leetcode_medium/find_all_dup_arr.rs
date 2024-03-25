fn abs<T: std::ops::Sub<Output = T> + std::cmp::PartialOrd>(x: T, y: T) -> T {
    if x < y {
        return y - x;
    }
    x - y
}
fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for i in 0..nums.len() {
        let x = abs(nums[i], 0) as usize;
        if nums[x - 1] < 0 {
            ans.push(x as i32);
        }
        nums[x - 1] *= -1;
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::find_duplicates;
    #[test]
    fn run_tc1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        assert_eq!(find_duplicates(nums), vec![2, 3]);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 1, 2];
        assert_eq!(find_duplicates(nums), vec![1]);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![1];
        assert_eq!(find_duplicates(nums), vec![]);
    }
}
fn main() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    assert_eq!(find_duplicates(nums), vec![2, 3]);
}
