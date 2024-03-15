fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut zeros = 0;
    let mut product = 1;
    for &x in nums.iter() {
        if x == 0 {
            zeros += 1;
            continue;
        }
        product *= x;
    }
    if zeros == 1 {
        nums.into_iter()
            .map(|x| if x == 0 { product } else { 0 })
            .collect()
    } else if zeros == 0 {
        nums.into_iter().map(|x| product / x).collect()
    } else {
        vec![0; nums.len()]
    }
}

#[cfg(test)]
pub mod tests {
    use crate::product_except_self;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(nums), vec![24, 12, 8, 6]);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![-1, 1, 0, -3, 3];
        assert_eq!(product_except_self(nums), vec![0, 0, 9, 0, 0]);
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(product_except_self(nums), vec![24, 12, 8, 6]);
}
