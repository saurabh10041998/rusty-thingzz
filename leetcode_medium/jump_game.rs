fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len() - 1;
    let mut idx = n - 1;
    for i in (0..nums.len()).rev() {
        if i + nums[i] as usize  >= idx {
            idx = i;
        }
    }
    idx == 0
}

#[cfg(test)]
pub mod tests { 
    use crate::can_jump;
    #[test]
    fn run_tc1() {
        let nums = vec![2,3,1,1,4];
        assert_eq!(can_jump(nums), true);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![3,2,1,0,4];
        assert_eq!(can_jump(nums), false);
    }
}

fn main() {
    let nums = vec![2,3,1,1,4];
    assert_eq!(can_jump(nums), true);
}
