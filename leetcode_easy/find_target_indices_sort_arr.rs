fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.sort();
    let mut result = Vec::new();
    for (i, &num) in nums.iter().enumerate() {
        if num == target {
            result.push(i as i32);
        }
    }
    result
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 2;
        let result = target_indices(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 3;
        let result = target_indices(nums, target);
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn run_tc3() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 5;
        let result = target_indices(nums, target);
        assert_eq!(result, vec![4]);
    }
}

fn main() {}
