fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    let mut low = 0;
    let mut high = nums.len() - 1;
    while low < high {
        let mid = low
            + match high.checked_sub(low) {
                Some(offset) => offset / 2,
                None => unreachable!(),
            };
        if nums[mid] >= target {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    if nums[high] != target {
        return -1;
    } else {
        high as i32
    }
}

#[cfg(test)]
pub mod tests {
    use crate::search;
    #[test]
    fn run_tc1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(search(nums, target), 4);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(search(nums, target), -1);
    }
}

fn main() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    assert_eq!(search(nums, target), 4);
}
