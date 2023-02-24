use std::collections::BinaryHeap;

fn minimum_deviation(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut pq = BinaryHeap::new();
    for mut num in nums.into_iter() {
        if num % 2 == 1 {
            num *= 2;
        }
        pq.push(num);
        min = i32::min(min, num);
    }
    let mut max_dev = i32::MAX;
    while let Some(mut max) = pq.pop() {
        max_dev = i32::min(max_dev, max - min);
        // We encountered maximum odd here, that means,
        // We must have divided the even (may be repeatedly by 2) which is scaled from odd
        // Now min_val is even and max_val is odd, this is minimum
        // max_dev you can get, because of the rule
        // that even is scaled down and odd is scaled up..
        // further operations widens the `max_dev`       
        if max % 2 == 1 {
            break;
        }
        max /= 2;
        min = i32::min(min, max); 
        pq.push(max);
    }
    max_dev
}


#[cfg(test)]
pub mod tests {
    use crate::minimum_deviation;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(minimum_deviation(nums), 1);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![4, 1, 5, 20, 3];
        assert_eq!(minimum_deviation(nums), 3);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![2, 10, 8];
        assert_eq!(minimum_deviation(nums), 3);
    }

}
fn main() {
    let nums = vec![4, 1, 5, 20, 3];
    assert_eq!(minimum_deviation(nums), 3);
}