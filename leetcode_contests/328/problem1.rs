use std::ops::Sub;

fn abs<T: PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x > y {
        return x - y;
    }
    y - x
}
fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut dig_sum = 0;
    for &n in nums.iter() {
        let tmp: i32 = format!("{}", n)
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .iter()
            .sum();
        dig_sum += tmp;
    }
    abs(sum, dig_sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 15, 6, 3];
        assert_eq!(difference_of_sum(nums), 9);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(difference_of_sum(nums), 0);
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}
fn main() {
    println!("Hello, world!");
}
