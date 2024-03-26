// Most satisfying number of iterations
const ITERATIONS: usize = 69;
fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let len = nums.len() as i32;
    for _iter in 0..ITERATIONS {
        for i in 0..nums.len() {
            let x = nums[i];
            if x >= 1 && x <= len {
                nums.swap(x as usize - 1, i);
            }
        }
    }
    for i in 0..nums.len() {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    len + 1
}

#[cfg(test)]
pub mod tests {
    use crate::first_missing_positive;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 0];
        assert_eq!(first_missing_positive(nums), 3);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![3, 4, -1, 1];
        assert_eq!(first_missing_positive(nums), 2);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![7, 8, 9, 11, 12];
        assert_eq!(first_missing_positive(nums), 1);
    }
}

fn main() {
    let nums = vec![7, 8, 9, 11, 12];
    assert_eq!(first_missing_positive(nums), 1);
}
