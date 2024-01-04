use std::collections::HashMap;

fn min_operations(nums: Vec<i32>) -> i32 {
    let freq = nums.into_iter().fold(HashMap::new(), |mut accum, num| {
        accum.entry(num).and_modify(|f| *f += 1).or_insert(1);
        accum
    });
    let mut res = 0;
    for (_, v) in freq {
        if v == 1 {
            return -1;
        }
        let modulus = v % 3;
        match modulus {
            0 => res += v / 3,
            1 => res += 2 + (v - 4) / 3,
            2 => res += 1 + (v - 2) / 3,
            _ => unreachable!(),
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::min_operations;
    #[test]
    fn run_tc1() {
        let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];
        assert_eq!(min_operations(nums), 4);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![2, 1, 2, 2, 3, 3];
        assert_eq!(min_operations(nums), -1);
    }
}

fn main() {
    let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];
    assert_eq!(min_operations(nums), 4);
}
