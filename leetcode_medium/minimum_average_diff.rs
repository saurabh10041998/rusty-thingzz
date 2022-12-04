use std::ops::Sub;

fn abs<T: PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x > y {
        return x - y;
    }
    y - x
}

fn minimum_average_difference(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i64;
    let mut total_sum: i64 = 0;
    for c in nums.iter() {
        total_sum += *c as i64;
    }

    let (mut abs_diff, mut ans, mut prefix_sum) = (i32::MAX, 0, 0 as i64);
    for (i, c) in nums.iter().enumerate() {
        prefix_sum += *c as i64;
        let left_avg = prefix_sum / (i + 1) as i64;
        let mut right_avg = total_sum - prefix_sum;
        if i as i64 != n - 1 {
            right_avg = right_avg / (n - i as i64 - 1);
        }
        let diff = abs(left_avg, right_avg);
        if diff < abs_diff as i64 {
            abs_diff = diff as i32;
            ans = i as i32;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::minimum_average_difference;
    #[test]
    fn run_tc1() {
        let nums = vec![2, 5, 3, 9, 5, 3];
        assert_eq!(minimum_average_difference(nums), 3);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![0];
        assert_eq!(minimum_average_difference(nums), 0);
    }
}

fn main() {
    let nums = vec![2, 5, 3, 9, 5, 3];
    assert_eq!(minimum_average_difference(nums), 3);
}
