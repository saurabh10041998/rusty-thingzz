use std::collections::BinaryHeap;

fn get_ceil_div_3(x: i32) -> i32 { 
    x / 3 + (x % 3 != 0) as i32
}

fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    let mut nums = nums;
    let mut score = 0;
    let mut pq = nums.drain(..).collect::<BinaryHeap<i32>>();
    for i in 0..k {
        let ele = pq.pop().unwrap();
        score += ele as i64; 
        pq.push(get_ceil_div_3(ele))
    } 
    score      
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![10,10,10,10,10];
        let k = 5;
        assert_eq!(max_kelements(nums, k), 50);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1,10,3,3,3];
        let k = 3;
        assert_eq!(max_kelements(nums, k), 17);
    }
    #[test]
    fn run_tc3() {
        assert_eq!(true, true);
    }
}
fn main() {
    println!("Hello, world!");
}
