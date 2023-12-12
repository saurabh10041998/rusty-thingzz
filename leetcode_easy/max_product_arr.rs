fn max_product(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold(vec![i32::MIN + 1, i32::MIN], |accum, n| {
            if n > accum[0] {
                vec![n, accum[0]]
            } else if n > accum[1] {
                vec![accum[0], n]
            } else {
                accum
            }
        })
        .into_iter()
        .map(|x| x - 1)
        .product()
}

#[cfg(test)]
pub mod tests {
    use crate::max_product;
    #[test]
    fn run_tc1() {
        let nums = vec![3, 4, 5, 2];
        assert_eq!(max_product(nums), 12);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 5, 4, 5];
        assert_eq!(max_product(nums), 16);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![10, 2, 5, 2];
        assert_eq!(max_product(nums), 36)
    }
}

fn main() {
    let nums = vec![1, 5, 4, 5];
    assert_eq!(max_product(nums), 16);
}
