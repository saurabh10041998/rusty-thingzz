use std::collections::HashMap;

fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let mut hs = HashMap::new();
    for &t in tasks.iter() {
        hs.entry(t).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut ans = 0;
    for (_k, v) in hs {
        if v == 1 {
            return -1;
        }
        ans += (v as f64 / 3 as f64).ceil() as i32;
    }
    ans
}
#[cfg(test)]
pub mod tests {
    use crate::minimum_rounds;
    #[test]
    fn run_tc1() {
        let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
        assert_eq!(minimum_rounds(tasks), 4);
    }

    #[test]
    fn run_tc2() {
        let tasks = vec![2, 3, 3];
        assert_eq!(minimum_rounds(tasks), -1);
    }
}
fn main() {
    let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
    assert_eq!(minimum_rounds(tasks), 4);
}
