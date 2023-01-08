fn maximum_count(nums: Vec<i32>) -> i32 {
    let mut neg = 0;
    let mut _zeros = 0;
    let mut pos = 0;
    for &n in nums.iter() {
        if n < 0 {
            neg += 1;
        }else if n == 0 {
            _zeros += 1;
        } else { 
            pos += 1;
        }
    }
    i32::max(neg, pos)
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![-2,-1,-1,1,2,3];
        assert_eq!(maximum_count(nums), 3);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![-3,-2,-1,0,0,1,2];
        assert_eq!(maximum_count(nums), 3);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![5,20,66,1314];
        assert_eq!(maximum_count(nums), 4);
    }
}
fn main() {
    println!("Hello, world!");
}
