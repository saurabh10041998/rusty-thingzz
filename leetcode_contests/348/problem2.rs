fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut first = 0;
    let mut last = 0;
    for (idx,&x) in nums.iter().enumerate()  {
        if x == 1 {
            first = idx;
        }
        if x == n as i32 {
            last = idx;
        }
    }
    if last < first {
        (n - 1 - last + first - 1) as i32 
    } else {
        (n - 1 - last + first) as i32
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![2,1,4,3];
        assert_eq!(semi_ordered_permutation(nums), 2);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![2,4,1,3];
        assert_eq!(semi_ordered_permutation(nums), 3);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![1,3,4,2,5];
        assert_eq!(semi_ordered_permutation(nums), 0);
    }
}

fn main() {
    println!("Hello, world!");
}