fn converge(weights: &Vec<i32>, days: i32, cap: i32) -> bool {
    let mut max_days = 1;
    let mut curr_cap = 0;
    for &w in weights {
        if curr_cap + w > cap {
            max_days += 1;
            curr_cap = 0;
        }
        curr_cap += w;
    }
    max_days <= days
}

fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    for &w in weights.iter() {
        left = i32::max(left, w);
        right += w;
    }
    let mut ans = -1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if converge(&weights, days, mid) {
            ans = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::ship_within_days;
    #[test]
    fn run_tc1() {
        let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let days = 5;
        assert_eq!(ship_within_days(weights, days), 15);
    }
    #[test]
    fn run_tc2() {
        let weights = vec![3, 2, 2, 4, 1, 4];
        let days = 3;
        assert_eq!(ship_within_days(weights, days), 6);
    }
    #[test]
    fn run_tc3() {
        let weights = vec![1, 2, 3, 1, 1];
        let days = 4;
        assert_eq!(ship_within_days(weights, days), 3);
    }
}

fn main() {
    let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let days = 5;
    assert_eq!(ship_within_days(weights, days), 15);
}
