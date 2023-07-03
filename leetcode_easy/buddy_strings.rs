use std::collections::HashMap;
fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    let mut hs = HashMap::new();
    let mut diffs = vec![];
    for (x, y) in s.chars().zip(goal.chars()) {
        if x != y {
            diffs.push((x,y));
        }
        hs.entry(x).and_modify(|val| *val += 1).or_insert(1);
    }
    if diffs.is_empty() {
        return hs.iter().any(|(_,&y)| y > 1);
    } else if diffs.len() == 2 {
        let (x1, y1) = diffs[0];
        let (x2, y2) = diffs[1];
        return x1 == y2 && x2 == y1;
    }
    false
}

#[cfg(test)]
pub mod tests {
    use crate::buddy_strings;
    #[test]
    fn run_tc1() {
        let s = String::from("ab");
        let goal = String::from("ba");
        assert!(buddy_strings(s,goal));
    }
    #[test]
    fn run_tc2() {
        let s = String::from("ab");
        let goal = String::from("ab");
        assert_eq!(buddy_strings(s,goal), false);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("aa");
        let goal = String::from("aa");
        assert_eq!(buddy_strings(s,goal), true);
    }
}
fn main() {
    let s = String::from("aa");
    let goal = String::from("aa");
    assert!(buddy_strings(s,goal));
}
