use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Number(i32);

fn digit_sum(n: i32) -> i32 {
    let mut m = n;
    let mut sum = 0;
    while m > 0 {
        sum += m % 10;
        m /= 10;
    }
    sum
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let x = digit_sum(self.0);
        let y = digit_sum(other.0);
        if x == y {
            return self.0.cmp(&other.0);
        }
        x.cmp(&y)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Number {}

fn min_swaps(mut nums: Vec<i32>) -> i32 {
    let mut sorted: Vec<Number> = nums.iter().map(|&x| Number(x)).collect();
    sorted.sort();
    //print!("{:?}", sorted);
    let mut pos = HashMap::new();
    for i in 0..nums.len() {
        pos.insert(nums[i], i);
    }

    let mut swap = 0;
    for i in 0..nums.len() {
        if nums[i] != sorted[i].0 {
            let to_swap = pos[&sorted[i].0];
            pos.insert(nums[i], to_swap);
            nums.swap(i, to_swap);
            pos.insert(sorted[i].0, i);
            swap += 1;
        }
    }
    swap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tc1() {
        assert_eq!(min_swaps(vec![37, 100]), 1);
    }

    #[test]
    fn run_tc2() {
        assert_eq!(min_swaps(vec![22, 14, 33, 7]), 0);
    }

    #[test]
    fn run_tc3() {
        assert_eq!(min_swaps(vec![18, 43, 34, 16]), 2);
    }
}

fn main() {}
