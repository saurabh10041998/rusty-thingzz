fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in nums {
        result = result ^ i;
    }
    result
}

#[cfg(test)]
pub mod tests {
    use crate::single_number;
    #[test]
    fn run_tc1() {
        let nums = vec![2,2,1];
        assert_eq!(single_number(nums), 1);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![4,1,2,1,2];
        assert_eq!(single_number(nums), 4);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![1];
        assert_eq!(single_number(nums), 1);
    }
}

fn main() {
    println!("Hello, world!");
    let nums = vec![1];
    assert_eq!(single_number(nums), 1);
}
