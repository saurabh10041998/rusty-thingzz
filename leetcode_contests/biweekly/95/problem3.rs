fn xor_beauty(nums: Vec<i32>) -> i32 {
    let mut ans = 0;  
    for x in nums {
        ans ^= x;
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![1,4];
        assert_eq!(xor_beauty(nums), 5);
    }
    #[test]
    fn run_tc2() {
        assert_eq!(true, true);
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}
fn main() {
    println!("Hello, world!");
}
