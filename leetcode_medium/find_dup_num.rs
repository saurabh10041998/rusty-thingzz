fn find_duplicate(nums: Vec<i32>) -> i32 {
    let (mut slow, mut fast) = (0, 0);
    loop {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        if slow == fast {
            break;
        }
    }
    let mut slow = 0;
    loop {
        slow = nums[slow] as usize;
        fast = nums[fast] as usize;
        if slow == fast {
            break;
        }
    }
    slow as i32
}

#[cfg(test)]
pub mod tests {
    use crate::find_duplicate;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate(nums), 2);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(find_duplicate(nums), 3);
    }
}

fn main() {
    let nums = vec![3, 1, 3, 4, 2];
    assert_eq!(find_duplicate(nums), 3);
}
