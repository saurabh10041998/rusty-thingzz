use std::collections::{HashMap, VecDeque};

fn remove_duplicate_letters(s: String) -> String {
    let mut mp = HashMap::new(); // char -> idx
    let mut in_queue_mp: HashMap<char, bool> = HashMap::new(); // char -> bool
    
    // record the highest index for character
    for (i, c) in s.chars().enumerate() {
        mp.entry(c).and_modify(|idx| *idx = i).or_insert(i);
    }
    let mut q = VecDeque::new();
    for (i, c) in s.chars().enumerate() {
        match in_queue_mp.get(&c) {
            Some(&val) if val => {}
            _ => {
                while !q.is_empty() {
                    let b = *q.back().unwrap();
                    if c < b && *mp.get(&b).unwrap() > i {
                        in_queue_mp.entry(b).and_modify(|val| *val = false);
                        q.pop_back();
                    } else {
                        break;
                    }
                }
                in_queue_mp.insert(c, true);
                q.push_back(c);
            }
        }
    }
    let mut ans = String::new();
    while let Some(c) = q.pop_front() {
        ans.push(c);
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::remove_duplicate_letters;
    #[test]
    fn run_tc1() {
        let s = String::from("bcabc");
        assert_eq!(remove_duplicate_letters(s), String::from("abc"));
    }
    #[test]
    fn run_tc2() {
        let s = String::from("cbacdcbc");
        assert_eq!(remove_duplicate_letters(s), String::from("acdb"));
    }
}

fn main() {
    let s = String::from("cbacdcbc");
    assert_eq!(remove_duplicate_letters(s), String::from("acdb"));
}
