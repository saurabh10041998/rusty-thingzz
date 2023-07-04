fn single_number(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for bit in 0..32 {
        let mut sum = 0;
        for &num in nums.iter() {
            if (num >> bit) & 1 == 1 {
                sum += 1;
                sum %= 3;
            }
        }
        if sum != 0 {
            ans |= sum << bit;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::single_number;
    #[test]
    fn run_tc1() {
        let nums = vec![2, 2, 3, 2];
        assert_eq!(single_number(nums), 3);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        assert_eq!(single_number(nums), 99);
    }
}

fn main() {
    let nums = vec![0, 1, 0, 1, 0, 1, 99];
    assert_eq!(single_number(nums), 99);
}
