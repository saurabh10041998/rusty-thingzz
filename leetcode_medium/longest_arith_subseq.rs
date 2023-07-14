use std::collections::HashMap;

fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
    let mut dp = HashMap::new();
    let mut ans = 0;
    for x in arr {
        match dp.get(&(x - difference)) {
            Some(&y) => {
                dp.entry(x).and_modify(|v| *v = y + 1).or_insert(y + 1);
            }
            None => {
                dp.entry(x).or_insert(1);
            }
        }
        ans = i32::max(ans, *dp.get(&x).unwrap());
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::longest_subsequence;
    #[test]
    fn run_tc1() {
        let arr = vec![1, 2, 3, 4];
        let difference = 1;
        assert_eq!(longest_subsequence(arr, difference), 4);
    }
    #[test]
    fn run_tc2() {
        let arr = vec![1, 3, 5, 7];
        let difference = 1;
        assert_eq!(longest_subsequence(arr, difference), 1);
    }
    #[test]
    fn run_tc3() {
        let arr = vec![1, 5, 7, 8, 5, 3, 4, 2, 1];
        let difference = -2;
        assert_eq!(longest_subsequence(arr, difference), 4);
    }
}

fn main() {
    let arr = vec![1, 5, 7, 8, 5, 3, 4, 2, 1];
    let difference = -2;
    assert_eq!(longest_subsequence(arr, difference), 4);
}
