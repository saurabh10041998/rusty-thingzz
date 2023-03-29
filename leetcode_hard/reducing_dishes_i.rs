use std::collections::HashMap;
fn helper(
    satisfaction: &Vec<i32>,
    idx: usize,
    time: usize,
    dp: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if idx == satisfaction.len() {
        return 0;
    }
    match dp.get(&(idx, time)) {
        Some(val) => return *val,
        None => {}
    };
    let include =
        satisfaction[idx] * (time as i32 + 1) + helper(satisfaction, idx + 1, time + 1, dp);
    let exclude = 0 + helper(satisfaction, idx + 1, time, dp);
    let ans = i32::max(include, exclude);
    dp.entry((idx, time)).or_insert(ans);
    ans
}

fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
    let mut satisfaction = satisfaction;
    satisfaction.sort();
    let mut dp = HashMap::new();
    helper(&satisfaction, 0, 0, &mut dp)
}

#[cfg(test)]
pub mod tests {
    use crate::max_satisfaction;
    #[test]
    fn run_tc1() {
        let satisfaction = vec![-1, -8, 0, 5, -9];
        assert_eq!(max_satisfaction(satisfaction), 14);
    }
    #[test]
    fn run_tc2() {
        let satisfaction = vec![4, 3, 2];
        assert_eq!(max_satisfaction(satisfaction), 20);
    }
    #[test]
    fn run_tc3() {
        let satisfaction = vec![-1, -4, -5];
        assert_eq!(max_satisfaction(satisfaction), 0);
    }
}

fn main() {
    let satisfaction = vec![-1, -8, 0, 5, -9];
    assert_eq!(max_satisfaction(satisfaction), 14);
}
