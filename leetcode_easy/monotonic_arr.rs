fn is_monotonic(nums: Vec<i32>) -> bool {
    let (mut dec, mut inc) = (true, true);
    let mut stack = vec![];
    stack.push(nums[0]);
    for x in nums.into_iter().skip(1) {
        if *stack.last().unwrap() > x {
            dec = false;
        }
        if *stack.last().unwrap() < x {
            inc = false;
        }
        stack.push(x);
    }
    dec || inc
}

#[cfg(test)]
pub mod tests {
    use crate::is_monotonic;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 2, 3];
        assert_eq!(is_monotonic(nums), true);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![6, 5, 4, 4];
        assert_eq!(is_monotonic(nums), true);
    }

    #[test]
    fn run_tc3() {
        let nums = vec![1, 3, 2];
        assert_eq!(is_monotonic(nums), false);
    }
}

fn main() {
    let nums = vec![1, 3, 2];
    assert_eq!(is_monotonic(nums), false);
}
