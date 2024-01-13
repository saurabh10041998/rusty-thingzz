use std::collections::HashMap;
fn min_steps(s: String, t: String) -> i32 {
    let (m1, m2) = s.chars().into_iter().zip(t.chars().into_iter()).fold(
        (HashMap::new(), HashMap::new()),
        |(mut m1, mut m2), (x, y)| {
            m1.entry(x).and_modify(|f| *f += 1).or_insert(1usize);
            m2.entry(y).and_modify(|f| *f += 1).or_insert(1usize);
            (m1, m2)
        },
    );
    let mut steps = 0;
    for c in 'a'..='z' {
        let f1 = match m1.get(&c) {
            Some(val) => *val,
            None => 0,
        };
        let f2 = match m2.get(&c) {
            Some(val) => *val,
            None => 0,
        };
        if let Some(diff) = f1.checked_sub(f2) {
            steps += diff;
        }
    }
    steps as i32
}

#[cfg(test)]
pub mod tests {
    use crate::min_steps;
    #[test]
    fn run_tc1() {
        let s = String::from("bab");
        let t = String::from("aba");
        assert_eq!(min_steps(s, t), 1);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("leetcode");
        let t = String::from("practice");
        assert_eq!(min_steps(s, t), 5);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("anagram");
        let t = String::from("mangaar");
        assert_eq!(min_steps(s, t), 0);
    }
}

fn main() {
    let s = String::from("anagram");
    let t = String::from("mangaar");
    assert_eq!(min_steps(s, t), 0);
}
