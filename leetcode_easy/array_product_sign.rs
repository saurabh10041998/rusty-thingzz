fn array_sign(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(1i32, |num, x| {
        num * if x > 0 {
            1
        } else if x < 0 {
            -1
        } else {
            0
        }
    })
}

#[cfg(test)]
pub mod tests {
    use crate::array_sign;
    #[test]
    fn run_tc1() {
        let nums = vec![-1, -2, -3, -4, 3, 2, 1];
        assert_eq!(array_sign(nums), 1);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 5, 0, 2, -3];
        assert_eq!(array_sign(nums), 0);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![-1, 1, -1, 1, -1];
        assert_eq!(array_sign(nums), -1);
    }
}

fn main() {
    let nums = vec![-1, 1, -1, 1, -1];
    assert_eq!(array_sign(nums), -1);
}
